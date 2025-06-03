// src/config.rs
use serde::Deserialize;
use dotenv::dotenv;
use std::env;

#[derive(Debug, Deserialize, Clone)] // Clone jika perlu di-pass ke AppState
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub maps_api_key: String,
    pub server_address: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv().ok(); // Muat .env file

        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            jwt_secret: env::var("JWT_SECRET")?,
            maps_api_key: env::var("Maps_API_KEY")?,
            server_address: env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                            .unwrap_or_else(|_| "3000".to_string())
                            .parse::<u16>()
                            .expect("SERVER_PORT must be a valid u16 number"),
        })
    }
}