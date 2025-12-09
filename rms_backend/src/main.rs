use std::io::Write;
use std::net::{TcpListener, TcpStream};

fn main() {
    // Binding
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Server running on 127.0.0.1:7878");

    // Iteration over an infinite stream of connection attempts
    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                eprint!("Failed to establish connection: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("New Connection Established!");

    let _ = stream.write_all(b"Hello from Rust!");
}
