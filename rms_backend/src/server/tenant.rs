use std::error::Error;
use std::io::Write;
use std::net::TcpStream;

use crate::{
    entities::{notice, request},
    ops::extractor,
    server::server::{SessionStore, get_session_id},
};

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
