use crate::user::{ User,Id};

pub fn extract_data(text: String) -> Result<User, String> {

    let mut first_name_buffer = String::new();
    let mut last_name_buffer = String::new();
    let mut role_buffer = String::new();
    let mut email_buffer = String::new();
    let mut phone_number_buffer = String::new();
    let mut password_buffer = String::new();

    if let Some((header, body)) = text.split_once("\r\n\r\n") {

        for pair in body.split("&") {

            if let Some((key, value)) = pair.split_once("="){

                match key {
                    "first_name" => {
                        first_name_buffer = value.to_string();
                    }

                    "last_name" => {
                        last_name_buffer = value.to_string();
                    }

                    "role" => {
                        role_buffer = value.to_string();
                    }

                    "email" => {
                        email_buffer = value.to_string();
                    }

                    "phone_number" => {
                        phone_number_buffer = value.to_string();
                    }

                    "password" => {
                        password_buffer = value.to_string();
                    }

                    _ => {}
                }
            } else {
                    return Err("Failed to match keys and values".to_string());
            }

        }
    } else {
        return Err("Failed to split header and body".to_string());
    }

    let user_id = Id::generate_id();
    Ok(User::new(
        user_id,
        first_name_buffer,
        last_name_buffer,
        role_buffer,
        email_buffer,
        phone_number_buffer,
        password_buffer,
    )?)
}
