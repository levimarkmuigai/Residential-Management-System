use uuid::Uuid;

use crate::{db::utils::get_client, entities::caretaker::Details};

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
