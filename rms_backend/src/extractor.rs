use crate::user::{ User,Id, UserCredentials};

pub fn extract_data(text: String) -> Result<User, String> {

    let mut first_name_buffer = String::new();
    let mut last_name_buffer = String::new();
    let mut role_buffer = String::new();
    let mut email_buffer = String::new();
    let mut phone_number_buffer = String::new();
    let mut password_buffer = String::new();

    for pair in text.split("&") {

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
                        email_buffer = value.to_string().replace("%40", "@");
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


pub fn data_for_auth(text: String) -> Result<UserCredentials, String> {

    let mut email_buffer = String::new();
    let mut password_buffer = String::new();

        for pair in text.split("&") {

            if let Some((key, value)) = pair.split_once("=") {

                match key {
                    "email" => {
                      email_buffer = value.to_string().replace("%40", "@");
                    }

                    "password" => {
                        password_buffer = value.to_string();
                    }

                    _ => {}
                }
            }
        }
    Ok(UserCredentials::new(
            email_buffer,
            password_buffer
    )?)
}
