use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, prelude::*};
use std::fs;

use crate::signup;
use crate::login;
use crate::extractor;

pub fn run_server() {

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("Server is running on port 8080...");

    for stream in listener.incoming() {
        println!("Connection Established.");

        handle_connection(stream.unwrap());
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);

    let header: Vec<String> = buf_reader
        .by_ref()
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

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
            handle_auth(body_str.to_string(), &mut stream);
        }

        line if line.starts_with("GET /login") => {
            println!("Route: Login being handled...");
            serve_static_file("login.html", "text/html", stream);
        }

        line if line.starts_with("GET /landlord") => {
            println!("Route: Welcome Landlord......");
            serve_static_file("landlord.html", "text/html", stream);
        }

        line if line.starts_with("GET /caretaker") => {
            println!("Routing: Welcome User");
            serve_static_file("caretaker.html", "text/html", stream);
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

fn handle_auth(text: String, stream: &mut TcpStream) {

    println!("Handling... {}", text);

    let user_result = extractor::data_for_auth(text);

    if let Ok(user) = user_result.as_ref() {
        println!("User Extracted: {:?}", user);
    }

    match login::auth(user_result.unwrap()) {
        Ok(response) => {
            println!("Successfully logged in: {}", response);
            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(e) => {
            println!("Login failed: {}", e);
        }
    }
}

fn serve_static_file(file_path: &str, content_type: &str, mut stream: TcpStream) {
    match fs::read_to_string(file_path) {
        Ok(content) => {
            let response = format!("HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\n\r\n{}",
                content.len(), content);

            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(_) => {
            let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}

