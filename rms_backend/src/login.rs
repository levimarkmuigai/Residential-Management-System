use crate::db;
use crate::user::UserCredentials;

use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2
};

pub fn auth(user: UserCredentials) -> Result<String, String> {

    let email = user.email.value();
    let password_attempt = user.password.value();

    let user = db::find_user(email).ok_or_else(|| "User not found".to_string())?;

    let db_password_hash = user.password.value();

    let parsed_hash = PasswordHash::new(&db_password_hash)
        .map_err(|e| format!("Invalid hash format in DB: {}", e))?;


    if Argon2::default().verify_password(password_attempt.as_bytes(), &parsed_hash).is_err() {
        println!("Invalid password attempt for: {}", email);
        return Err("Login failed".to_string());
    }

    println!("Login Successful for: {}", email);

    let role = user.role.value();

    let location = match role {
        "Landlord" => "/landlord",
        "Caretaker" => "/caretaker",
        "Tenant"   => "/tenant",
        _          => "/login",
    };

    let status_line = "HTTP/1.1 303 See Other";

    let response = format!("{status_line}\r\nLocation: {location}\r\n\r\n");

    println!("{}", response);

    Ok(response)
}

