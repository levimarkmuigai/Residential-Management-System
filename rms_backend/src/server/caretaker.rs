use std::{error::Error, io::Write, net::TcpStream};

use crate::{
    db,
    entities::caretaker,
    ops::extractor,
    server::server::{SessionStore, get_session_id},
};

pub fn handle_caretaker_details(
    body: String,
    stream: &mut TcpStream,
    sessions: &SessionStore,
    header: &str,
) -> Result<(), Box<dyn Error>> {
    let mut caretaker_dto = extractor::caretaker::dto(body)?;

    let session_id = get_session_id(header);

    caretaker_dto.session_id = session_id;

    match caretaker::update_table(caretaker_dto, sessions) {
        Ok(response) => {
            stream.write_all(response.as_bytes())?;
            stream.flush()?;
        }
        Err(e) => {
            eprintln!("ERROR IN CARETAKER TABLE: MODULE caretaker.rs {:?}", e);
            let error_response = "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\n\r\n";
            stream.write_all(error_response.as_bytes())?;
            stream.flush()?;
        }
    }

    Ok(())
}

pub fn get_dash(
    sessions: &SessionStore,
    stream: &mut TcpStream,
    header: &str,
) -> Result<(), Box<dyn Error>> {
    if let Some(sid) = get_session_id(header) {
        let lock = sessions.lock().map_err(|_| "FAILED TO LOCK SESSIONS")?;

        if let Some(id) = lock.get(&sid) {
            match caretaker::build_dashboard(*id) {
                Ok(html) => {
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                        html.len(),
                        html
                    );

                    stream.write_all(response.as_bytes())?;
                    stream.flush()?;
                }
                Err(e) => {
                    eprintln!("ERROR LOADING CARETAKER DASHBOARD: {}", e);

                    stream.write_all(b"HTTP/1.1 303 See Other\r\nLocation: /login\r\n\r\n")?;
                    stream.flush()?;
                }
            }

            return Ok(());
        }
    }

    stream.write_all(b"HTTP/1.1 303 See Other\r\nLocation: /login\r\n\r\n")?;
    stream.flush()?;
    Ok(())
}

pub fn assign_tenant(body: String, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    println!("DEBUG: {}", body);

    match extractor::caretaker::assignment_params(body) {
        Ok((unit_id, tenant_id)) => match db::tenant::assign_unit(unit_id, tenant_id) {
            Ok(_) => {
                let response = "HTTP/1.1 303 See Other\r\nLocation: /caretaker\r\n\r\n";
                stream.write_all(response.as_bytes())?;
                stream.flush()?;
            }
            Err(e) => {
                eprintln!("Database assignment failed: {}", e);
                let error_response =
                    "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\n\r\n";
                stream.write_all(error_response.as_bytes())?;
                stream.flush()?;
            }
        },
        Err(e) => {
            eprintln!("Extraction failed: {}", e);
            let error_response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
            stream.write_all(error_response.as_bytes())?;
            stream.flush()?;
        }
    }
    Ok(())
}
