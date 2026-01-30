use crate::db;
use crate::user::user::User;

pub fn signup(user: User) -> Result<String, String> {

    match db::insert_user(user) {
        Ok(_) => {

            println!("Routing to: Login Page");
        }
        Err(e) => {

            println!("Failed to insert user. {}", e);
        }
    }

    let status_line = "HTTP/1.1 303 See Other";

    let location = "Location: /login";

    let response = format!("{status_line}\r\n{location}\r\n\r\n");

    Ok(response)
}

