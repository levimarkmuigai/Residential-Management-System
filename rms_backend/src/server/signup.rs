use std::{error::Error, io::Write, net::TcpStream};

use crate::ops::{extractor::auth, signup::signup};

pub fn handle_signup(body: String, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    print!("Handling... {}", body);

    let user = auth::user_data(body)?;
    println!("User Extracted: \n{:?}", user);

    let response = signup(user)?;
    println!("Successfully signed up. Redirecting to login...");

    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}
