use std::net::TcpListener;
use std::io::Read;

pub fn http() {

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    print!("Listerner running on port 8080");

    match listener.accept() {
        Ok((mut stream, _addr)) => {
            println!("New connection established.");

            let mut buffer = [0; 1024];

            match stream.read(&mut buffer) {
                Ok(read_bytes) => {
                    let string = String::from_utf8_lossy(&buffer[..read_bytes]);
                }
                Err(e) => println!("Failed to read from stream: {}", e),
            }
        }
        Err(e) => println!("Connection failed: {}", e),
    } 
}
