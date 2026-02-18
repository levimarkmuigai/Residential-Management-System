use crate::user::user::UserCredentials;
use crate::{db, server::server::SessionStore};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordVerifier},
};
use uuid::Uuid;

pub fn auth(user: UserCredentials, sessions: &SessionStore) -> Result<String, String> {
    let email = user.email.value();
    let password_attempt = user.password.value();

    let user = db::user::find_by_email(email).ok_or_else(|| "User not found".to_string())?;

    let db_password_hash = user.password.value();

    let parsed_hash = PasswordHash::new(&db_password_hash)
        .map_err(|e| format!("Invalid hash format in DB: {}", e))?;

    if Argon2::default()
        .verify_password(password_attempt.as_bytes(), &parsed_hash)
        .is_err()
    {
        println!("Invalid password attempt for: {}", email);
        return Err("Login failed".to_string());
    }

    println!("Login Successful for: {}", email);

    let session_key = generate_session(sessions, *user.id.value());

    let role = user.role.value();

    let location = match role {
        "Landlord" => "/landlord",
        "Caretaker" => "/caretaker",
        "Tenant" => "/tenant",
        _ => "/login",
    };

    let status_line = "HTTP/1.1 303 See Other";

    let response = format!(
        "{status_line}\r\nLocation: {location}\r\nSet-Cookie: session_id={session_key}; HttpOnly; Path=/; SameSite=Lax\r\n\r\n"
    );

    println!("{}", response);

    Ok(response)
}

fn generate_session(sessions: &SessionStore, user_id: Uuid) -> String {
    let session_key = Uuid::new_v4().to_string();

    let mut lock = sessions.lock().unwrap();
    lock.insert(session_key.clone(), user_id);

    session_key
}
