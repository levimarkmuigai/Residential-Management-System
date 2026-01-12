use crate::db;

use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2
};
pub fn login(email: &str, password_attempt: String) -> Result<(), String> {

    let user = db::find_user_credentials(email);

    let binding = match user {
        Some(u) => u,
        None => {

            return Err("User not found".to_string());

        }
    };

    let db_password = binding.password.value();

    let hashed_password = PasswordHash::new(&db_password)
        .map_err(|e| e.to_string())?;

    if Argon2::default().verify_password(password_attempt.as_bytes(), &hashed_password).is_ok() {
        println!("Login Successful");

        let for_user_role = db::find_user(email.clone());

        let for_user_binding = match for_user_role {
            Some(u) => u,
            None => {
                return Err("User not found".to_string());
            }
        };

    } else {
        return Err("Login failed".to_string());
    }

    Ok(())
}

