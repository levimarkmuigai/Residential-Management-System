use uuid::Uuid;

use crate::{
    db::utils::get_client,
    entities::{notice::Notice, request::Request},
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

pub fn send_request(request: Request) -> Result<(), String> {
    let mut client = get_client().unwrap();

    let mut transaction = client.transaction().unwrap();

    let tenant_id = request.tenant_id.value();

    let fetch_statement = "SELECT id FROM units WHERE tenant_id = $1 LIMIT 1";

    let row = transaction
        .query_opt(fetch_statement, &[&tenant_id])
        .map_err(|e| e.to_string())?;

    let unit_id: Uuid = row
        .map(|r| r.get(0))
        .ok_or("Row empty!Unit id not found!")?;

    let sql_statement = "
        INSERT INTO maintenance_requests
        (id,tenant_id,unit_id,issue_type,description,priority,status)
         VALUES($1,$2,$3,$4,$5,$6,$7)";

    transaction
        .execute(
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
        )
        .map_err(|e| e.to_string())?;

    transaction.commit().map_err(|e| e.to_string())?;

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
