use std::{error::Error, io::Write, net::TcpStream};

use crate::{server::extractors::auth, services};

pub fn handle_signup(body: String, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    print!("Handling... {}", body);

    let user = auth::user_data(body)?;
    println!("User Extracted: \n{:?}", user);

    let response = services::auth::signup(user)?;
    println!("Successfully signed up. Redirecting to login...");

    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}
