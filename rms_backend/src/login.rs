use crate::db;

use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2
};

pub fn login(email: &str, password_attempt: &str) -> Result<String, String> {

    let user = db::find_user(email).ok_or_else(|| "User not found".to_string())?;

    let db_password_hash = user.password.value();

    let parsed_hash = PasswordHash::new(&db_password_hash)
        .map_err(|e| format!("Invalid hash format in DB: {}", e))?;

    let clean_attempt = password_attempt.trim();
    if Argon2::default().verify_password(clean_attempt.as_bytes(), &parsed_hash).is_err() {
        println!("Invalid password attempt for: {}", email);
        return Err("Login failed".to_string());
    }

    println!("Login Successful for: {}", email);

    let role = user.role.value();
    let location = match role {
        "Landlord" => "/landlord",
        "Tenant"   => "/tenant",
        _          => "/landing_page",
    };

    let response = format!(
        "HTTP/1.1 303 See Other\r\n\
        Location: {}\r\n\
        Content-Length: 0\r\n\r\n",
        location
    );

    Ok(response)
}

