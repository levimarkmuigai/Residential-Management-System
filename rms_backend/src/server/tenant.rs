use std::io::Write;
use std::net::TcpStream;
use std::{error::Error, fs};

use crate::{
    entities::{notice, request, tenant},
    ops::extractor,
    server::server::{SessionStore, get_session_id},
};

pub fn get_dash(
    sessions: &SessionStore,
    stream: &mut TcpStream,
    header: &str,
) -> Result<(), Box<dyn Error>> {
    let user_id = get_session_id(header).and_then(|sid| {
        let lock = sessions
            .lock()
            .unwrap_or_else(|poison_error| poison_error.into_inner());

        lock.get(&sid).cloned()
    });
    if let Some(id) = user_id {
        match tenant::dash_display(id) {
            Ok(data) => {
                let (rows, notice_div) = data;

                let mut html = fs::read_to_string("tenant.html")?;

                html = html.replace("{{rows}}", &rows);
                html = html.replace("{{notice_div}}", &notice_div);

                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-type: text/html\r\nContent-length: {}\r\n\r\n{}",
                    html.len(),
                    html
                );

                stream.write_all(response.as_bytes())?;
                stream.flush()?;
            }
            Err(e) => {
                eprintln!("CRITICAL ERROR in dash_display: {:?}", e);

                let error_html =
                    "HTTP/1.1 500 Internal Server Error\r\n\r\nDashboard generation failed.";
                let _ = stream.write_all(error_html.as_bytes());
                let _ = stream.flush();

                return Err(e);
            }
        }
    } else {
        stream.write_all(
            b"HTTP/1.1 303 See Other\r\nLocation: /login
                         \r\n\r\n",
        )?;
    }
    Ok(())
}

pub fn handle_maintenance_request(
    body: String,
    stream: &mut TcpStream,
    sessions: &SessionStore,
    header: &str,
) -> Result<(), Box<dyn Error>> {
    println!("Request {} recieved.", body);

    let mut request_dto = extractor::request::dto(body)?;

    let session_id = get_session_id(header);
    request_dto.session_id = session_id;

    println!(
        "Request processing for session: {:?}",
        request_dto.session_id
    );

    match request::send_request(request_dto, sessions) {
        Ok(response) => {
            stream.write_all(response.as_bytes())?;
            stream.flush()?;
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            let error_response = "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\n\r\n";
            stream.write_all(error_response.as_bytes())?;
            stream.flush()?;
        }
    }
    Ok(())
}

pub fn handle_vacation_notice(
    body: String,
    stream: &mut TcpStream,
    sessions: &SessionStore,
    header: &str,
) -> Result<(), Box<dyn Error>> {
    println!("Handling, {}", body);

    let mut notice_dto = extractor::notice::dto(body)?;

    let session_id = get_session_id(header);
    notice_dto.session_id = session_id;

    println!("DTO ready {:?}", notice_dto);

    match notice::send_notice(notice_dto, sessions) {
        Ok(response) => {
            stream.write_all(response.as_bytes())?;
            stream.flush()?;
        }
        Err(e) => {
            eprintln!("Error in notice module: {:?}", e);
            let error_response = "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\n\r\n";
            stream.write_all(error_response.as_bytes())?;
            stream.flush()?;
        }
    }
    Ok(())
}
