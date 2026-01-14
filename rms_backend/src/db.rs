use postgres::{Client, NoTls};

use std::env;

use crate::user::{
    User,
    Id,
    FirstName,
    LastName,
    PhoneNumber,
    Email,
    Password,
    Role
};

pub fn get_client() -> Result<Client, postgres::Error> {

    let database_url = env::var("DATABASE_URL")
        .expect("Failed to find url.");

    Ok(Client::connect(&database_url, NoTls)?)
}

pub fn insert_user(user: User) -> Result<(), String> {

    let mut client = get_client().map_err(|e| e.to_string())?;

    let sql_statement = "
    INSERT INTO users (id, role, first_name, last_name, email, phone_number, password)
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

    let mut client = get_client().map_err(|e| e.to_string()).ok()?;

    let sql_statement = "
        SELECT id,role,first_name, last_name,email,phone_number,password FROM users WHERE email = $1";

    let result = client.query_opt(sql_statement, &[&email]).unwrap();

    result.map(|row| User {
        id: Id{ id: row.get("id")},
        role: Role{ raw: row.get("role")},
        first_name: FirstName{ raw: row.get("first_name")},
        last_name: LastName{ raw: row.get("last_name")},
        email: Email{ raw: row.get("email")},
        phone_number: PhoneNumber{ raw: row.get("phone_number")},
        password: Password{ raw: row.get("password")},
    })
}

