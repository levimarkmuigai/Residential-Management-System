pub mod db;
pub mod entities;
pub mod ops;
pub mod server;
pub mod user;

use std::error::Error;

use dotenvy;

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    println!("Starting server, Welcome to RMS BACKEND");

    server::server::run_server()?;

    println!("Hello Rust");

    Ok(())
}
