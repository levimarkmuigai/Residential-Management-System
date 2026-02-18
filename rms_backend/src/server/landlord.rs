use std::{error::Error, fs, io::Write, net::TcpStream};

use crate::{
    db,
    entities::landlord,
    ops::extractor,
    server::server::{SessionStore, get_session_id},
};

pub fn handle_landlord_profile_update(
    body: String,
    stream: &mut TcpStream,
    sessions: &SessionStore,
    header: &str,
) -> Result<(), Box<dyn Error>> {
    println!("Handling Landlord Profile Update {}", body);

    let mut landlord_dto = extractor::landlord::dto(body)?;

    let session_id = get_session_id(header);

    landlord_dto.session_id = session_id;

    match landlord::update_profile(landlord_dto, &sessions) {
        Ok(response) => {
            stream.write_all(response.as_bytes())?;
            stream.flush()?;
        }

        Err(e) => {
            eprintln!("ERROR: {}", e);

            let response = "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\n\r\n";
            stream.write_all(response.as_bytes())?;
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
    println!("Route: Welcome Landlord......");

    if let Some(sid) = get_session_id(header) {
        let lock = sessions.lock().map_err(|_| "FAILED TO LOCK SESSION")?;

        if let Some(landlord_uuid) = lock.get(&sid) {
            let (rows, c_options, b_options) = landlord::manage_buildings(*landlord_uuid)?;

            let mut html = fs::read_to_string("landlord.html")?;

            html = html.replace(" {{BUILDING_ROWS}}", &rows);
            html = html.replace("{{CARETAKER_OPTIONS}}", &c_options);
            html = html.replace("{{BUILDINGS}}", &b_options);

            let response = format!(
                "HTTP/1.1 200 OK
                        \r\nContent-Type: text/html\r\nContent-Length: {}
                        \r\n\r\n{}",
                html.len(),
                html
            );

            stream.write_all(response.as_bytes())?;
        } else {
            stream.write_all(
                b"HTTP/1.1 303 See Other\r\nLocation: /login
                         \r\n\r\n",
            )?;
        }
    }

    Ok(())
}

pub fn assign_caretaker(body: String, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    match extractor::landlord::assignment_params(body) {
        Ok((building_id, caretaker_id)) => {
            match db::building::assign_caretaker(building_id, caretaker_id) {
                Ok(_) => {
                    let response = "HTTP/1.1 303 See Other\r\nLocation: /landlord\r\n\r\n";
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
            }
        }
        Err(e) => {
            eprintln!("Extraction failed: {}", e);
            let error_response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
            stream.write_all(error_response.as_bytes())?;
            stream.flush()?;
        }
    }
    Ok(())
}
