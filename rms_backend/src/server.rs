use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use crate::data;
use crate::login;
use crate::signup;

pub fn start_server() {

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("Listerner running on port 8080");

    loop {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection established.");

                    thread::spawn(|| {
                        handle_connection(stream);
                    });
                }
                Err(e) => println!("Connection failed: {}", e),
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 4096];

    match stream.read(&mut buffer) {
        Ok(read_bytes) => {
            if read_bytes == 0 {
                return;
            }

            let request_text = String::from_utf8_lossy(&buffer[..read_bytes]).to_string();
            println!("Recieved request: \n{}", request_text);

            let first_line = request_text.lines().next().unwrap_or("");
            let mut parts = first_line.split_whitespace();

            let method = parts.next().unwrap_or("");
            let path = parts.next().unwrap_or("");

            match (method, path) {
                ("POST", "/register") => {
                    println!("Handling registration");
                    handle_registration(request_text, &mut stream);
                }

                ("POST", "/login") => {
                    println!("Handling login");
                    handle_login(request_text, &mut stream);
                }

                ("GET", "/landlord") => {
                    println!("Routing to Landlord dashboard.");

                    let response = "HTTP/1.1 200 OK\r\n\r\nWelcome Landlord";

                    stream.write_all(response.as_bytes()).unwrap();
                }

                ("GET", "/caretaker") => {
                    println!("Routing to Caretaker dashboard.");

                    let response = "HTTP/1.1 200 Ok\r\n\r\nWelcime Caretaker";

                    stream.write_all(response.as_bytes()).unwrap();
                }

                ("GET", "tenant") => {
                    println!("Routing to Tenant dashboard.");

                    let _response = "HTTP/1.1 200 Ok\r\n\r\nWelcome Tenant";
                }

                _ => {
                    println!("404 sent for path: {}", path);
                    let response = "HTTP/1.1 404 Not Found\r\n\r\nUnknown path";

                    stream.write_all(response.as_bytes()).unwrap();
                }
            }
        }
        Err(e) => println!("Failed to established connection: {}", e),
    }
}

fn handle_registration(request_text: String, stream: &mut TcpStream) {
    match data::extract_data(request_text) {
        Ok(user) => {

            println!("User extracted {:?}", user);
            
            match signup::signup(user) {
                Ok(redirect) => {

                    println!("Sign up Success! Redirecting to: {}", redirect);
                    stream.write_all(redirect.as_bytes()).unwrap();
                }
                Err(_) => {

                    let response = "HTTP/1.1 500 Error\r\n\r\nDatabase Error";
                    stream.write_all(response.as_bytes()).unwrap();
                }
            }

        },
        Err(_) => {

            let response = "HTTP/1.1 400 Bad Request\r\n\r\nInvalid data";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}

fn handle_login(request_text: String, stream: &mut TcpStream) {
    match data::extract_credentials(request_text) {
        Ok(credentials) => {

            println!("Authentication data extracted {:?}", credentials);

            let email = credentials.email.raw.trim();
            let password = credentials.password.raw.trim();

            match login::login(&email,&password) {
                Ok(redirect) => {
                    println!("Login Success! Redirecting to: {}", redirect);
                    stream.write_all(redirect.as_bytes()).unwrap();
                }
                Err(_) => {

                    let response = "HTTP/1.1 401 Unauthoried\r\n\r\nInvalid Credentials";
                    stream.write_all(response.as_bytes()).unwrap();
                }
            }
        },
        Err(_) => {

            let response = "HTTP/1.1 400 Bad Request\r\n\r\nInvalid data";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}

