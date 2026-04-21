use std::{error::Error, io::Write, net::TcpStream};

use crate::{server::extractors::auth, services};

pub fn handle_signup(body: String, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    print!("Handling... {}", body);

    let user = match auth::user_data(body) {
        Ok(u) => u,
        Err(e) => {
            println!("Error extracting user: {}", e);
            let error_response = "HTTP/1.1 400 Bad Request\r\n\r\n";
            stream.write_all(error_response.as_bytes())?;
            stream.flush()?;

            return Ok(());
        }
    };
    println!("User Extracted: \n{:?}", user);

    let response = match services::auth::signup(user) {
        Ok(res) => res,
        Err(e) => {
            println!("Error saving user: {}", e);
            let error_response = "HTTP/1.1 500 Internal Server Error\r\n\r\n";
            stream.write_all(error_response.as_bytes())?;
            stream.flush()?;

            return Ok(());
        }
    };
    println!("Successfully signed up");
    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}
