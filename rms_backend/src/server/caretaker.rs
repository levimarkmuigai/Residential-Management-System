use std::{error::Error, fs, io::Write, net::TcpStream};

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
    let user_id = get_session_id(header).and_then(|sid| {
        let lock = sessions
            .lock()
            .unwrap_or_else(|poison_error| poison_error.into_inner());
        lock.get(&sid).copied()
    });

    if let Some(id) = user_id {
        match caretaker::build_dashboard(id) {
            Ok(data) => {
                let (a_rows, n_rows, u_rows, t_rows) = data;

                let mut html = fs::read_to_string("caretaker.html")?;

                html = html.replace("{{urgent_rows}}", &u_rows);

                html = html.replace("{{tasks_rows}}", &t_rows);

                html = html.replace("{{assign_rows}}", &a_rows);

                html = html.replace("{{notice_rows}}", &n_rows);

                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-type: text/html\r\nContent-length: {}\r\n\r\n{}",
                    html.len(),
                    html
                );

                stream.write_all(response.as_bytes())?;
                stream.flush()?;
            }
            Err(e) => {
                eprintln!("CRITICAL ERROR in build_dashboard: {:?}", e);

                let error_html =
                    "HTTP/1.1 500 Internal Server Error\r\n\r\nDashboard generation failed.";
                let _ = stream.write_all(error_html.as_bytes());
                let _ = stream.flush();

                return Err(e);
            }
        }
    } else {
        stream.write_all(b"HTTP/1.1 303 See Other\r\nLocation: /login\r\n\r\n")?;
    }
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
