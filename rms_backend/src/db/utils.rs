use std::env;

use postgres::{Client, NoTls};

pub fn get_client() -> Result<Client, postgres::Error> {
    let database_url = env::var("DATABASE_URL").expect("Failed to find url.");

    Ok(Client::connect(&database_url, NoTls)?)
}
