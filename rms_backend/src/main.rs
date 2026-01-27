mod server;
mod db;
mod extractor;
mod user;
mod login;
mod signup;

use dotenvy;

fn main() {

    dotenvy::dotenv().ok();

    println!("Starting server, Welcome to RMS BACKEND");

    server::run_server();

    println!("Hello Rust");
}
