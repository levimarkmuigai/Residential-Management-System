use std::error::Error;

use uuid::Uuid;

use crate::{
    db::utils::get_client,
    entities::{caretaker::Details, notice::CaretakerNotice, request::CaretakerTask},
};

pub fn save_details(details: Details) -> Result<(), String> {
    let mut client = get_client().unwrap();

    let sql = "
        UPDATE caretakers
        SET national_id = $1, hired_at =$2
        WHERE user_id = $3
        ";

    client
        .execute(
            sql,
            &[&details.national_id, &details.hired_at, &details.user_id],
        )
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn get() -> Result<Vec<(Uuid, String)>, String> {
    let mut client = get_client().unwrap();

    let sql = "
        SELECT u.id, u.first_name
        FROM users u
        INNER JOIN caretakers c ON u.id = c.user_id
        LEFT JOIN buildings b ON u.id = b.caretaker_id
        WHERE u.role = 'Caretaker' AND b.id is NULL";

    let rows = client.query(sql, &[]).map_err(|e| {
        eprint!("DEBUG: DB ERROR {:?}", e);
        e.to_string()
    })?;

    let mut caretakers = Vec::new();

    for row in rows {
        let id = row.get(0);
        let name = row.get(1);

        caretakers.push((id, name));
    }

    Ok(caretakers)
}

pub fn get_building(caretaker_id: Uuid) -> Result<Uuid, String> {
    let mut client = get_client().unwrap();

    let sql = "
        SELECT id FROM buildings WHERE caretaker_id = $1 LIMIT 1
        ";

    let row = client
        .query_opt(sql, &[&caretaker_id])
        .map_err(|e| e.to_string())?;

    match row {
        Some(result) => Ok(result.get(0)),
        None => Err("No building found for the caretaker".to_string()),
    }
}

pub fn get_vacancy_count(user_id: Uuid) -> Result<String, Box<dyn Error>> {
    let mut client = get_client()?;

    let sql = "SELECT COUNT(*)
        FROM units u
        JOIN buildings b ON b.id = u.building_id
        JOIN caretakers c ON c.user_id = b.caretaker_id
        WHERE c.user_id = $1";

    let row = client.query_one(sql, &[&user_id])?;

    let count = row.get::<_, i64>(0).to_string();

    Ok(count)
}

pub fn get_urgent_tasks(user_id: Uuid) -> Result<Vec<CaretakerTask>, Box<dyn Error>> {
    let mut client = get_client()?;

    let sql = "
    SELECT
    r.id,
    u.unit_number AS u_number,
    r.description,
    r.created_at,
    r.status,
    usr.first_name AS name
    FROM maintenance_requests r
    JOIN units u ON r.unit_id = u.id
    JOIN buildings b ON u.building_id = b.id
    JOIN users usr ON r.tenant_id = usr.id
    WHERE b.caretaker_id = $1
    AND r.priority = $2
    ORDER BY r.created_at";

    let priority = "high";

    let rows = client.query(sql, &[&user_id, &priority])?;

    let urgent_tasks = rows
        .into_iter()
        .map(|r| CaretakerTask {
            id: r.get("id"),
            unit_no: r.get("u_number"),
            description: r.get("description"),
            timestamp: r.get("created_at"),
            status: r.get("status"),
            tenant_name: r.get("name"),
        })
        .collect();

    Ok(urgent_tasks)
}

pub fn get_other_tasks(user_id: Uuid) -> Result<Vec<CaretakerTask>, Box<dyn Error>> {
    let mut client = get_client()?;

    let sql = "
    SELECT
    r.id,
    u.unit_number AS u_number,
    r.description,
    r.created_at,
    r.status,
    usr.first_name AS name
    FROM maintenance_requests r
    JOIN units u ON r.unit_id = u.id
    JOIN buildings b ON u.building_id = b.id
    JOIN users usr ON r.tenant_id = usr.id
    WHERE b.caretaker_id = $1
    AND r.priority = $2
    ORDER BY r.created_at";

    let priority = "medium";

    let rows = client.query(sql, &[&user_id, &priority])?;

    let other_tasks = rows
        .into_iter()
        .map(|r| CaretakerTask {
            id: r.get("id"),
            unit_no: r.get("u_number"),
            description: r.get("description"),
            timestamp: r.get("created_at"),
            status: r.get("status"),
            tenant_name: r.get("name"),
        })
        .collect();

    Ok(other_tasks)
}

pub fn get_notices(user_id: Uuid) -> Result<Vec<CaretakerNotice>, Box<dyn Error>> {
    let mut client = get_client()?;

    let sql = "
        SELECT
        n.id,
        n.tenant_id,
        n.notice_date,
        n.status,
        u.unit_number AS unit_no,
        usr.first_name AS tenant_name
        FROM vacation_notices n
        JOIN units u ON u.tenant_id = n.tenant_id
        JOIN buildings b ON b.id = u.building_id
        JOIN users usr ON usr.id = n.tenant_id
        WHERE b.caretaker_id = $1
        ORDER BY n.submitted_at";

    let rows = client.query(sql, &[&user_id])?;

    let notices = rows
        .into_iter()
        .map(|row| CaretakerNotice {
            id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            notice_date: row.get("notice_date"),
            status: row.get("status"),
            unit_no: row.get("unit_no"),
            tenant_name: row.get("tenant_name"),
        })
        .collect();

    Ok(notices)
}
