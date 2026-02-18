use uuid::Uuid;

use crate::{db::utils::get_client, entities::building::BuildingRow};

pub fn get_stats(landlord_id: Uuid) -> Result<Vec<BuildingRow>, String> {
    let mut client = get_client().unwrap();

    let sql = "
            SELECT 
            b.id,
            b.name,
            b.total_units_count,
            COUNT(u.id) FILTER(WHERE u.is_occupied = true) AS occupied_units
                FROM buildings b
                LEFT JOIN units u ON b.id = u.building_id
                WHERE b.landlord_id = $1
                GROUP BY b.id, b.name, b.total_units_count
                ORDER BY b.name ASC";

    let rows = client.query(sql, &[&landlord_id]).map_err(|e| {
        eprintln!("POSTGRES ERROR: {:?}", e);
        e.to_string()
    })?;

    println!(
        "DEBUG: Found {} buildings for landlord {}",
        rows.len(),
        landlord_id
    );

    let mut building_list = Vec::new();

    for row in rows {
        building_list.push(BuildingRow {
            id: row.get(0),
            name: row.get(1),
            units: row.get(2),
            occupied_units: row.get::<_, i64>(3) as i32,
        });
    }

    Ok(building_list)
}

pub fn assign_caretaker(building_id: Uuid, caretaker_id: Uuid) -> Result<(), String> {
    let mut client = get_client().unwrap();

    let sql = "
            UPDATE buildings
            SET caretaker_id = $1
            WHERE id = $2
            ";

    client
        .execute(sql, &[&caretaker_id, &building_id])
        .map_err(|e| {
            eprintln!("ERROR INSERTIMG CARETAKER: {:?}", e);
            e.to_string()
        })?;

    Ok(())
}

pub fn get_vacancies(building_id: Uuid) -> Result<Vec<(Uuid, i32)>, String> {
    let mut client = get_client().unwrap();

    let sql = "
        SELECT id,unit_number FROM units 
        WHERE building_id = $1 AND is_occupied = false
        ";

    let rows = client
        .query(sql, &[&building_id])
        .map_err(|e| e.to_string())?;

    let mut vacant_units = Vec::new();

    for row in rows {
        let id = row.get(0);
        let unit_number = row.get(1);

        vacant_units.push((id, unit_number));
    }

    Ok(vacant_units)
}

pub fn get_caretaker(building_id: Uuid) -> Result<Option<String>, String> {
    let mut client = get_client().unwrap();

    let sql = "
        SELECT u.first_name 
        FROM buildings b
        LEFT JOIN caretakers c ON b.caretaker_id = c.user_id
        LEFT JOIN users u ON c.user_id = u.id
        WHERE b.id = $1";

    let result = client.query_opt(sql, &[&building_id]).map_err(|e| {
        eprintln!("DATABASE ERROR FETCHING CARETAKER ID: {:?}", e);
        e.to_string()
    })?;

    match result {
        Some(row) => {
            let name = row.get(0);

            Ok(name)
        }
        None => Err("Caretaker id not found".to_string()),
    }
}
