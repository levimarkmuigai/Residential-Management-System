use std::{
    collections::HashMap,
    error::Error,
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

use uuid::Uuid;

use crate::server::{building, caretaker, landlord, login, signup, tenant};

pub type SessionStore = Arc<Mutex<HashMap<String, Uuid>>>;

pub fn run_server() -> Result<(), String> {
    let listener = TcpListener::bind("127.0.0.1:8080").map_err(|e| {
        println!("ERROR: {:?}", e);
        e.to_string()
    })?;

    let sessions: SessionStore = Arc::new(Mutex::new(HashMap::new()));

    for s in listener.incoming() {
        let stream = s.map_err(|e| {
            println!("ERROR: {:?}", e);
            e.to_string()
        })?;

        let sessions_clone = Arc::clone(&sessions);

        let _ = handle_connection(stream, sessions_clone);
    }

    Ok(())
}

fn parse_request(stream: &mut TcpStream) -> Result<(String, String, String, String), String> {
    let mut buf = BufReader::new(stream);

    let header_lines: Vec<String> = buf
        .by_ref()
        .lines()
        .map(|l| l.unwrap())
        .take_while(|l| !l.is_empty())
        .collect();

    let header = header_lines.join("\n");

    let request_line = header_lines.get(0).cloned().unwrap_or_default();

    let parts: Vec<&str> = request_line.split_whitespace().collect();

    let method = parts.get(0).unwrap_or(&"").to_string();

    let path = parts.get(1).unwrap_or(&"").to_string();

    let content_length = header_lines
        .iter()
        .find(|l| l.to_lowercase().starts_with("content-length:"))
        .and_then(|l| l.split_once(":"))
        .and_then(|(_, v)| v.trim().parse::<usize>().ok())
        .unwrap_or(0);

    let mut body_as_bytes = vec![0; content_length];

    buf.read_exact(&mut body_as_bytes).map_err(|e| {
        println!("ERROR: {:?}", e);

        e.to_string()
    })?;

    let body = String::from_utf8_lossy(&body_as_bytes).to_string();

    Ok((method, path, header, body))
}

pub fn get_session_id(header: &str) -> Option<String> {
    header
        .lines()
        .find(|line| line.to_lowercase().starts_with("cookie:"))
        .and_then(|line| {
            line.split(';')
                .find(|cookie_pair| cookie_pair.contains("session_id="))
                .and_then(|pair| pair.split_once("="))
                .map(|(_, v)| v.trim().to_string())
        })
}

pub fn handle_connection(mut stream: TcpStream, sessions: SessionStore) -> Result<(), String> {
    let (method, path, header, body) = parse_request(&mut stream)?;

    match method.as_str() {
        "GET" => match path.as_str() {
            "/login" => {
                let _ = serve_file("login.html", "text/html", stream);
            }
            "/landlord" => {
                let _ = landlord::get_dash(&sessions, &mut stream, &header);
            }
            "/caretaker" => {
                let _ = caretaker::get_dash(&sessions, &mut stream, &header);
            }
            p if p.ends_with(".css") => {
                let _ = serve_file(p.trim_start_matches('/'), "text/css", stream);
            }

            _ => println!("404 GET not found {}", path),
        },

        "POST" => match path.as_str() {
            "/signup" => {
                let _ = signup::handle_signup(body, &mut stream);
            }
            "/login" => {
                let _ = login::login(body, &mut stream, &sessions);
            }
            "/register_bulding" => {
                let _ =
                    building::handle_building_registration(body, &mut stream, &sessions, &header);
            }
            "/update_landlord" => {
                let _ =
                    landlord::handle_landlord_profile_update(body, &mut stream, &sessions, &header);
            }
            "/landlord/assign_caretaker" => {
                let _ = landlord::assign_caretaker(body, &mut stream);
            }
            "/caretaker_form" => {
                let _ = caretaker::handle_caretaker_details(body, &mut stream, &sessions, &header);
            }
            "/caretaker/assign_tenant" => {
                let _ = caretaker::assign_tenant(body, &mut stream);
            }
            "/maintenance_request" => {
                let _ = tenant::handle_maintenance_request(body, &mut stream, &sessions, &header);
            }
            "/vaction_notice" => {
                let _ = tenant::handle_vacation_notice(body, &mut stream, &sessions, &header);
            }

            _ => println!("404 POST not found: {}", path),
        },

        _ => println!("Unsupported Method: {}", method),
    }
    Ok(())
}

fn serve_file(
    file_path: &str,
    content_type: &str,
    mut stream: TcpStream,
) -> Result<(), Box<dyn Error>> {
    match fs::read_to_string(file_path) {
        Ok(content) => {
            let response = format!(
                "HTTP/1.1 200 Ok\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\n\r\n{}",
                content.len(),
                content
            );
            stream.write_all(response.as_bytes())?;
            stream.flush()?;

            Ok(())
        }
        Err(e) => {
            let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

            stream.write_all(response.as_bytes())?;
            stream.flush()?;

            Err(Box::new(e))
        }
    }
}
