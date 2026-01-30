use postgres::{Client, NoTls, Row};
use postgres::types::ToSql;
use uuid::Uuid;

use std::env;

use crate::building::Building;
use crate::landlord::Landlord;

use crate::user::{
    fields::{
    Id,
    Name,
    PhoneNumber,
    Email,
    Password,
    Role
    },
    user::User
};

pub fn get_client() -> Result<Client, postgres::Error> {

    let database_url = env::var("DATABASE_URL")
        .expect("Failed to find url.");

    Ok(Client::connect(&database_url, NoTls)?)
}

pub fn insert_user(user: User) -> Result<(), String> {

    let mut client = get_client().map_err(|e| e.to_string())?;

    let sql_statement = "
    INSERT INTO users (id, role, first_name, last_name,
        email, phone_number, password_hash)
        VALUES($1, $2, $3, $4, $5, $6, $7)";

    client.execute(sql_statement, &[
        &user.id.value(),
        &user.role.value(),
        &user.first_name.value(),
        &user.last_name.value(),
        &user.email.value(),
        &user.phone_number.value(),
        &user.password.value()])
    .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn find_user(email: &str) -> Option<User> {

    let mut client = get_client().expect("Failed to connect to DB");

    let sql_statement = "
        SELECT id,role,first_name, last_name,email,phone_number,password_hash 
        FROM users WHERE email = $1";

    let result = client.query_opt(sql_statement, 
        &[&email])
        .expect("Failed to fetch user.");

    result.map(|row: Row| User {
        id: Id{ id: row.get("id")},
        role: Role{ raw: row.get("role")},
        first_name: Name{ raw: row.get("first_name")},
        last_name: Name{ raw: row.get("last_name")},
        email: Email{ raw: row.get("email")},
        phone_number: PhoneNumber{ raw: row.get("phone_number")},
        password: Password{ raw: row.get("password_hash")},
    })
}

pub fn update_landlord(landlord: Landlord) -> Result<(), String> {
    let mut client = get_client().map_err(|e| e.to_string())?;

    let sql_statement = "INSERT INTO landlords(user_id, business_name)
        VALUEs($1, $2)";

    client.execute(sql_statement,
        &[&landlord.id.value() as &(dyn ToSql + Sync),
        &landlord.business_name.value()])
    .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn insert_buidling(building: Building) -> Result<(), String> {
    let mut client = get_client().map_err(|e| e.to_string())?;

    let  mut transaction = client.transaction()
        .map_err(|e| e.to_string())?;

    let sql_building = "
        INSERT INTO buildings(id,name,landlord_id,total_units_count)
       VALUES($1,$2,$3,$4)";

    let _row = transaction.execute(sql_building, &[
        &building.id.value(),
        &building.name.value(),
        &building.landlord_id.value(),
        &building.units
    ]).map_err(|e| e.to_string())?;

    let total_units = building.units;

    let sql_units = "
        INSERT INTO units(id, building_id, unit_number, is_occupied)
        VALUES($1,$2,$3,false)";

    let statement = transaction
        .prepare(sql_units).map_err(|e| e.to_string())?;

    let building_id = building.id.value();
    
    for i in 1..=total_units {

        let id = Uuid::new_v4();

        transaction.execute(&statement, &[
            &id,
            &building_id,
            &i
        ]).map_err(|e| e.to_string())?;
    }

    transaction.commit().map_err(|e| e.to_string())?;
    Ok(())
}

