use std::{error::Error, io::Write, net::TcpStream};

use crate::{
    entities::building,
    ops::extractor,
    server::server::{SessionStore, get_session_id},
};

pub fn handle_building_registration(
    body: String,
    stream: &mut TcpStream,
    sessions: &SessionStore,
    header: &str,
) -> Result<(), Box<dyn Error>> {
    println!("DEBUG: Handling...{}", body);

    let mut building_dto = extractor::building::dto(body)?;

    println!("Building Extracted...{:?}", building_dto);

    let session_id = get_session_id(header);
    building_dto.session_id = session_id;

    println!(
        "DEBUG: Building has recieved session Id ---> {:?}",
        building_dto
    );

    match building::save(building_dto, sessions) {
        Ok(response) => {
            stream.write_all(response.as_bytes())?;
            stream.flush()?;
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            let response = "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\n\r\n";
            stream.write_all(response.as_bytes())?;
            stream.flush()?;
        }
    }

    Ok(())
}
