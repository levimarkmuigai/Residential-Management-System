use std::error::Error;

use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::{
    db::utils::get_client,
    entities::{
        notice::{DisplayNotice, Notice},
        request::{DbRequest, Request},
    },
};

pub fn assign_unit(unit_id: Uuid, tenant_id: Uuid) -> Result<(), String> {
    let mut client = get_client().unwrap();

    let sql = "
            UPDATE units
            SET tenant_id = $1, is_occupied = true
            WHERE id = $2
            ";

    client
        .execute(sql, &[&tenant_id, &unit_id])
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn send_request(request: Request) -> Result<(), Box<dyn Error>> {
    let mut client = get_client()?;

    let mut transaction = client.transaction()?;

    let tenant_id = request.tenant_id.value();

    let fetch_statement = "SELECT id FROM units WHERE tenant_id = $1 LIMIT 1";

    let row = transaction.query_opt(fetch_statement, &[&tenant_id])?;

    let unit_id: Uuid = row
        .map(|r| r.get(0))
        .ok_or("Row empty!Unit id not found!")?;

    let sql_statement = "
        INSERT INTO maintenance_requests
        (id,tenant_id,unit_id,issue_type,description,priority,status)
         VALUES($1,$2,$3,$4,$5,$6,$7)";

    transaction.execute(
        sql_statement,
        &[
            &request.id.value(),
            &request.tenant_id.value(),
            &unit_id,
            &request.issue_type,
            &request.description,
            &request.priority,
            &request.status,
        ],
    )?;

    transaction.commit()?;

    Ok(())
}

pub fn send_notice(notice: Notice) -> Result<(), String> {
    let mut client = get_client().unwrap();

    let sql_statement = "INSERT INTO vacation_notices
        (id,tenant_id,notice_date,status) 
        VALUES($1,$2,$3,$4)";

    client
        .execute(
            sql_statement,
            &[
                &notice.id.value(),
                &notice.tenant_id.value(),
                &notice.date,
                &notice.status,
            ],
        )
        .map_err(|e| format!("DB error {:?}", e))?;

    Ok(())
}

pub fn get_unassigned() -> Result<Vec<(Uuid, String)>, String> {
    let mut client = get_client().unwrap();

    let sql = "
        SELECT u.id, u.first_name
        FROM users u
        INNER JOIN tenants t ON u.id = t.user_id
        LEFT JOIN units un ON u.id = un.tenant_id
        WHERE un.id is NULL
        ";

    let rows = client.query(sql, &[]).map_err(|e| e.to_string())?;

    let mut unassigned_tenants = Vec::new();

    for row in rows {
        let id = row.get(0);
        let name = row.get(1);

        unassigned_tenants.push((id, name));
    }

    Ok(unassigned_tenants)
}

pub fn get_requests(user_id: Uuid) -> Result<Vec<DbRequest>, Box<dyn Error>> {
    let mut client = get_client()?;

    let sql = "
    SELECT *
    FROM maintenance_requests 
    WHERE tenant_id=$1";

    let rows = client.query(sql, &[&user_id]).map_err(|e| {
        eprintln!("DATABASE ERROR: {:?}", e);
        e
    })?;

    let request = rows
        .into_iter()
        .map(|row| {
            let submitted_at: NaiveDateTime = row.get("created_at");

            DbRequest {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                unit_id: row.get("unit_id"),
                issue_type: row.get("issue_type"),
                description: row.get("description"),
                priority: row.get("priority"),
                status: row.get("status"),
                submitted_at,
            }
        })
        .collect();

    Ok(request)
}

pub fn get_notice(user_id: Uuid) -> Result<Option<DisplayNotice>, Box<dyn Error>> {
    let mut client = get_client()?;

    let sql = "
        SELECT id,status 
        FROM vacation_notices 
        WHERE tenant_id = $1
        LIMIT 1";

    let result = client.query_opt(sql, &[&user_id])?;

    let notice = result.map(|r| DisplayNotice {
        id: r.get(0),
        status: r.get(1),
    });

    Ok(notice)
}
