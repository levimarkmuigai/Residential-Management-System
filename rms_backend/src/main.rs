mod http;
mod db;
mod data;
mod user;

use dotenvy;

fn main() {

    dotenvy::dotenv().ok();

    println!("Starting server, Welcome to RMS BACKEND");

    http::start_server();

    println!("Hello Rust");
}
