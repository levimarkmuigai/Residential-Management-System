pub mod user;

mod building;
mod server;
mod db;
mod extractor;
mod login;
mod signup;
mod landlord;

use dotenvy;

fn main() {

    dotenvy::dotenv().ok();
 
    println!("Starting server, Welcome to RMS BACKEND");

    server::run_server();

    println!("Hello Rust");
}
