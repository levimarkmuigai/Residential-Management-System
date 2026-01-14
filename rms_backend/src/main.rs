mod http;
mod db;
mod data;
mod user;
mod login;
mod signup;

use dotenvy;

fn main() {

    dotenvy::dotenv().ok();

    println!("Starting server, Welcome to RMS BACKEND");

    http::start_server();

    println!("Hello Rust");
}
