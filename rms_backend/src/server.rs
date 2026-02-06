use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, prelude::*};
use std::fs;

use crate::{building,request, signup, notice};
use crate::login;
use crate::extractor;
use crate::landlord;

use uuid::Uuid;

use std::collections::HashMap;
use std::sync::{Arc,Mutex};

pub type SessionStore = Arc<Mutex<HashMap<String, Uuid>>>;

pub fn run_server() {

    let listener = TcpListener::bind("127.0.0.1:8080")
        .unwrap();

    let sessions: SessionStore = Arc::new(Mutex::new(HashMap::new()));

    println!("Server is running on port 8080...");

    for stream in listener.incoming() {
        let sessions_clone = Arc::clone(&sessions);

        println!("Connection Established.");

        handle_connection(stream.unwrap(), sessions_clone);
    }
}

pub fn get_session_id(request: &str) -> Option<String> {
    for line in request.lines() {
        if line.starts_with("Cookie:") && line.contains("session_id=") {
            return line.split("session_id=")
                .nth(1)
                .map(|s| s.split(";").next().unwrap_or(s)
                    .to_string());
        }
    }

    None
}

fn handle_connection(mut stream: TcpStream, sessions: SessionStore) {
    let mut buf_reader = BufReader::new(&mut stream);

    let header: Vec<String> = buf_reader
        .by_ref()
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let full_headers = header.join("\n");
    let current_session_id = get_session_id(&full_headers);

    println!("Users session id: {:?}", current_session_id);

    let content_length = header.iter()
        .find(|line| line.to_lowercase().starts_with("content-length:"))
        .and_then(|line| line.split_once(":"))
        .and_then(|(_, value)| value.trim().parse::<usize>().ok())
        .unwrap_or(0);

    let mut body_bytes = vec![0; content_length];
    buf_reader.read_exact(&mut body_bytes).unwrap();

    let body_str = String::from_utf8_lossy(&body_bytes);

    let request_line = header.get(0).map(|s| s.as_str()).unwrap_or("");
    println!("Here is your request_line: '{}'", request_line);

    match request_line {

        line if line.starts_with("POST /signup") => {
            println!("Route: Signup being handled...");
            handle_signup(body_str.to_string(), &mut stream);
        }

        line if line.starts_with("POST /login") => {
            println!("Request recieved, authenticating user.");
            handle_auth(body_str.to_string(), &mut stream, sessions);
        }

        line if line.starts_with("POST /update_landlord") => {
            println!("Request recieved, updating profile.");
            handle_landlord(body_str.to_string(), &mut stream,
            sessions, &full_headers);
        }

        line if line.starts_with("POST /register_b") => {
            println!("Request recieved, registering building.");
            handle_building(body_str.to_string(), &mut stream,
            sessions, &full_headers);
        }

        line if line.starts_with("POST /request") => {
            println!("Request recieved, sending request.");
            handle_request(body_str.to_string(), &mut stream, sessions,
                &full_headers);
            }

        line if line.starts_with("POST /vacate") => {
            println!("Vacation notice recieved. Wait for approval.");
            handle_vacate(body_str.to_string(), &mut stream, 
                sessions, &full_headers);
        }

        line if line.starts_with("GET /login") => {
            println!("Route: Login being handled...");
            serve_static_file("login.html", "text/html", stream);
        }

        line if line.starts_with("GET /landlord") => {
            println!("Route: Welcome Landlord......");

            if let Some(sid) = current_session_id {
                let lock = sessions.lock()
                    .unwrap();

                if let Some(landlord_uuid) = lock.get(&sid) {
                    let rows = building::manage_buildings
                        (*landlord_uuid).unwrap_or_else(|e| 
                            { 
                                println!("DEBUG: Error building table: {}", e); 
                                String::new()
                            });


                    let mut html = fs::read_to_string
                        ("landlord.html").unwrap();

                    html = html.replace(" {{BUILDING_ROWS}}", &rows);

                    let response = format!("HTTP/1.1 200 OK
                        \r\nContent-Type: text/html\r\nContent-Length: {}
                        \r\n\r\n{}",html.len(), html
                        );

                    stream.write_all(response.as_bytes()).unwrap();
                } else {
                    stream.write_all
                        (b"HTTP/1.1 303 See Other\r\nLocation: /login
                         \r\n\r\n").unwrap();
                }
            }
            serve_static_file("landlord.html", "text/html", stream);
        }

        line if line.starts_with("GET /caretaker") => {
            println!("Routing: Welcome Caretaker...");
            serve_static_file("caretaker.html", "text/html", stream);
        }

        line if line.contains("GET /tenant") => {
            println!("Routing: Welcome Tenant...");
            serve_static_file("tenant.html", "text/html", stream);
        }

        line if line.contains(".css") => {
            let filename = request_line
                .split_whitespace()
                .nth(1)
                .unwrap_or("")
                .trim_start_matches('/');
            serve_static_file(filename, "text/css", stream);
        }

        _ => {
            println!("Route: Not Found (404)");
        }
    }
}

fn handle_signup(text: String, stream: &mut TcpStream) {

    print!("Handling... {}", text);

    let user = extractor::extract_data(text).unwrap();
    println!("User Extracted: \n{:?}", user);

    let response = signup::signup(user);
    println!("Successfully signed up. Redirecting to login...");

    stream.write_all(response.unwrap().as_bytes()).unwrap();
}

fn handle_auth(text: String, stream: &mut TcpStream,
    sessions: SessionStore) {

    println!("Handling... {}", text);

    let user_result = extractor::data_for_auth(text);

    if let Ok(user) = user_result.as_ref() {
        println!("User Extracted: {:?}", user);
    }

    match login::auth(user_result.unwrap(), &sessions) {
        Ok(response) => {
            println!("Successfully logged in: {}", response);
            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(e) => {
            println!("Login failed: {}", e);
        }
    }
}

fn handle_landlord(text: String, stream: &mut TcpStream,
    sessions: SessionStore, raw_request: &str) {
    println!("Handling Landlord Profile Update {}", text);

    let mut landlord_dto = extractor::extract_landlord(text).unwrap();

    let session_id = get_session_id(raw_request);

    landlord_dto.session_id = session_id;

    match landlord::update_landlord(landlord_dto, &sessions) {
        Ok(response) => stream.write_all(response.as_bytes())
            .unwrap(),
        Err(e) => println!("Error: {}", e),
    }
}

fn handle_building(text: String, stream: &mut TcpStream,
    sessions: SessionStore, raw_request: &str) {
    println!("Handling...{}, registration", text);

    let mut building_dto = extractor::extract_building(text).unwrap();
    println!("Building Extracted...{:?}", building_dto);

    let session_id = get_session_id(raw_request);
    building_dto.session_id = session_id;

    print!("Building has recieved session Id: {:?}", building_dto);

    match building::insert_building(building_dto, &sessions) {
        Ok(response) => stream.write_all(response.as_bytes())
            .unwrap(),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn handle_request(text: String, stream: &mut TcpStream, 
    sessions: SessionStore, raw_request: &str) {
    println!("Request {} recieved.", text);

    let mut request_dto = extractor::extract_request(text);

    let session_id = get_session_id(raw_request);
    request_dto.session_id = session_id;

    println!("Request {:?} recieved: ", request_dto);

    match request::send_request(request_dto, &sessions) {
        Ok(response) => stream.write_all(response.as_bytes())
            .unwrap(),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn handle_vacate(text: String, stream: &mut TcpStream, 
    sessions: SessionStore, full_header: &str) {
    println!("Handling, {}", text);

    let mut notice_dto = extractor::extract_notice(text);

    let session_id = get_session_id(full_header);

    notice_dto.session_id = session_id;

    println!("DTO ready {:?}", notice_dto);

   match notice::send_notice(notice_dto, &sessions) {
       Ok(response) => stream.write_all(response.as_bytes()).unwrap(),
       Err(e) => println!("Error in notice module: {:?}", e),
   }

}

fn serve_static_file(file_path: &str, content_type: &str, mut stream: TcpStream) {
    match fs::read_to_string(file_path) {
        Ok(content) => {
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: 
                {content_type}\r\nContent-Length: {}\r\n\r\n{}",
                content.len(), content);

            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(_) => {
            let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}

