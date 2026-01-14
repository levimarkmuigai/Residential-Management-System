use crate::db;
use crate::user::User;

pub fn signup(user: User) -> Result<String, String> {

    let location = "/login";

    let response = format!(
        "HTTP/1.1 303 See Other\r\n\
        Location: {0}\r\n\
        Content-Length: 0\r\n\r\n",
        location
    );

    match db::insert_user(user) {
        Ok(_) => {

            println!("Routing to: {}", location);
            Ok(response)
        }
        Err(e) => {

            println!("Failed to inser user. {}", e);
            Err("Database insertion failed".to_string())
        }
    }
}

