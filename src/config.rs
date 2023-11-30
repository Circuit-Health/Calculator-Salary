use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct Config {
    pub server_address: String,
    // Add other configuration fields as needed
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv::dotenv().ok(); // Load .env file, if it exists

        let server_address = env::var("SERVER_ADDRESS")?;

        Ok(Config {
            server_address,
            // Initialize other fields in a similar way
        })
    }
}
