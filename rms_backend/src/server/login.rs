use crate::ops::extractor::auth;
use crate::ops::login;
use crate::server::server::SessionStore;
use std::error::Error;
use std::io::Write;
use std::net::TcpStream;

pub fn login(
    body: String,
    stream: &mut TcpStream,
    sessions: &SessionStore,
) -> Result<(), Box<dyn Error>> {
    println!("Handling... {}", body);

    let user_result = auth::auth_params(body)?;

    println!("DEBUG: Extracted ---> {:?}", user_result);

    match login::auth(user_result, sessions) {
        Ok(response) => {
            println!("Successfully logged in: {}", response);
            stream.write_all(response.as_bytes())?;
            stream.flush()?;
        }
        Err(e) => {
            println!("Login failed: {}", e);
            let error_response =
                "HTTP/1.1 401 Unauthorized\r\nLocation: /login\r\nContent-Length: 0\r\n\r\n";
            stream.write_all(error_response.as_bytes())?;
            stream.flush()?;
        }
    }

    Ok(())
}
