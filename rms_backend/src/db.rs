use postgres::types::ToSql;
use postgres::{Client, NoTls, Row};
use uuid::Uuid;

use std::env;

use crate::building::{Building, BuildingRow};
use crate::caretaker::Details;
use crate::landlord::Landlord;

use crate::notice::Notice;
use crate::request::Request;
use crate::user::{
    fields::{Email, Id, Name, Password, PhoneNumber, Role},
    user::User,
};

pub fn get_client() -> Result<Client, postgres::Error> {
    let database_url = env::var("DATABASE_URL").expect("Failed to find url.");

    Ok(Client::connect(&database_url, NoTls)?)
}

pub fn insert_user(user: User) -> Result<(), String> {
    let mut client = get_client().map_err(|e| e.to_string())?;

    let mut transaction = client.transaction().map_err(|e| e.to_string())?;

    let sql_statement = "
    INSERT INTO users (id, role, first_name, last_name,
        email, phone_number, password_hash)
        VALUES($1, $2, $3, $4, $5, $6, $7)";

    transaction
        .execute(
            sql_statement,
            &[
                &user.id.value(),
                &user.role.value(),
                &user.first_name.value(),
                &user.last_name.value(),
                &user.email.value(),
                &user.phone_number.value(),
                &user.password.value(),
            ],
        )
        .map_err(|e| e.to_string())?;

    let role = user.role.value();
    let id = user.id.value();

    match role {
        "Landlord" => transaction.execute(
            "
                INSERT INTO landlords(user_id, business_name) VALUES($1)",
            &[&id, &"Pending Update"],
        ),

        "Caretaker" => transaction.execute(
            "
                INSERT INTO caretakers(user_id, national_id) VALUES($1,$2)",
            &[&id, &format!("TEMP-{}", id.to_string()[..8].to_uppercase())],
        ),

        "Tenant" => transaction.execute(
            "
                INSERT INTO tenants(user_id, payment_status) VALUES($1, $2)",
            &[&id, &"up-to-date"],
        ),

        _ => Ok(0),
    }
    .map_err(|e| {
        eprint!("CRITICLA DATABASE ERROR: {:?}", e);
        format!("Role initialization failed: {}", e)
    })?;

    transaction.commit().map_err(|e| e.to_string())?;

    Ok(())
}

pub fn find_user(email: &str) -> Option<User> {
    let mut client = get_client().expect("Failed to connect to DB");

    let sql_statement = "
        SELECT id,role,first_name, last_name,email,phone_number,password_hash 
        FROM users WHERE email = $1";

    let result = client
        .query_opt(sql_statement, &[&email])
        .expect("Failed to fetch user.");

    result.map(|row: Row| User {
        id: Id { id: row.get("id") },
        role: Role {
            raw: row.get("role"),
        },
        first_name: Name {
            raw: row.get("first_name"),
        },
        last_name: Name {
            raw: row.get("last_name"),
        },
        email: Email {
            raw: row.get("email"),
        },
        phone_number: PhoneNumber {
            raw: row.get("phone_number"),
        },
        password: Password {
            raw: row.get("password_hash"),
        },
    })
}

pub fn update_landlord(landlord: Landlord) -> Result<(), String> {
    let mut client = get_client().map_err(|e| e.to_string())?;

    let sql_statement = "INSERT INTO landlords(user_id, business_name)
        VALUES($1, $2)";

    client
        .execute(
            sql_statement,
            &[
                &landlord.id.value() as &(dyn ToSql + Sync),
                &landlord.business_name.value(),
            ],
        )
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn insert_buidling(building: Building) -> Result<(), String> {
    let mut client = get_client().map_err(|e| e.to_string())?;

    let mut transaction = client.transaction().map_err(|e| e.to_string())?;

    let sql_building = "
        INSERT INTO buildings(id,name,landlord_id,total_units_count)
       VALUES($1,$2,$3,$4)";

    let _row = transaction
        .execute(
            sql_building,
            &[
                &building.id.value(),
                &building.name.value(),
                &building.landlord_id.value(),
                &building.units,
            ],
        )
        .map_err(|e| e.to_string())?;

    let total_units = building.units;

    let sql_units = "
        INSERT INTO units(id, building_id, unit_number, is_occupied)
        VALUES($1,$2,$3,false)";

    let statement = transaction.prepare(sql_units).map_err(|e| e.to_string())?;

    let building_id = building.id.value();

    for i in 1..=total_units {
        let id = Uuid::new_v4();

        transaction
            .execute(&statement, &[&id, &building_id, &i])
            .map_err(|e| e.to_string())?;
    }

    transaction.commit().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn insert_request(request: Request) -> Result<(), String> {
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

pub fn insert_notice(notice: Notice) -> Result<(), String> {
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

pub fn handle_caretaker_table(details: Details) -> Result<(), String> {
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

pub fn get_building_stats(landlord_id: Uuid) -> Result<Vec<BuildingRow>, String> {
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

pub fn get_caretakers() -> Result<Vec<(Uuid, String)>, String> {
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

pub fn get_caretakers_building(caretaker_id: Uuid) -> Result<Uuid, String> {
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

pub fn assign_caretaker_to_building(building_id: Uuid, caretaker_id: Uuid) -> Result<(), String> {
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

pub fn assign_tenant_to_unit(unit_id: Uuid, tenant_id: Uuid) -> Result<(), String> {
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

pub fn get_unassigned_tenant() -> Result<Vec<(Uuid, String)>, String> {
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

pub fn get_building_caretaker(building_id: Uuid) -> Result<Option<String>, String> {
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
