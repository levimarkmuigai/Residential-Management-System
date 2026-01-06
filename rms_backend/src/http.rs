use std::net::TcpListener;
use std::io::{Read, Write};
use crate::data;
use crate::db;

pub fn start_server() {

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    print!("Listerner running on port 8080");

    loop {
        match listener.accept() {
            Ok((mut stream, _addr)) => {
                println!("New connection established.");

                let mut buffer = [0; 4096];

                match stream.read(&mut buffer) {
                    Ok(read_bytes) => {
                        let string = String::from_utf8_lossy(&buffer[..read_bytes]);

                        let requested_text = string.to_string();

                        match data::extract_data(requested_text) {
                            Ok(user) => {
                                println!("User Extracted {:?}", user);

                                match db::insert_user(user) {
                                    Ok(_) => {

                                        println!("User saved to database!");

                                        let response = "HTTP/1.1 200 OK\r\n\r\nUSER successfully created!";
                                        stream.write_all(response.as_bytes()).unwrap();
                                        stream.flush().unwrap();
                                    }
                                    Err(e) => {

                                        println!("Database Error: {}", e);

                                        let response = "HTTP/1.1 500 Internal Server Error\r\n\r\nDatabase failure";
                                        stream.write_all(response.as_bytes()).unwrap();
                                        stream.flush().unwrap();
                                    }
                                }
                            }
                            Err(e) => {

                                println!("Failed to parse data {}", e);

                                let response = "HTTP/1.1 400 Bad Request\r\n\r\nInvalid form data";
                                stream.write_all(response.as_bytes()).unwrap();
                                stream.flush().unwrap();
                            }
                        }
                    }
                    Err(e) => println!("Failed to read from stream: {}", e),
                }
            }
            Err(e) => println!("Connection failed: {}", e),
        }
    }
}

