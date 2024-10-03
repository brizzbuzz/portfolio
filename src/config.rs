use std::env;
use rocket::fs::relative;

pub struct Config {
    pub asset_path: String,
    pub environment: String,
}

impl Config {
    pub fn build() -> Self {
        Config {
            asset_path: env::var("ASSET_PATH").unwrap_or(relative!("public").to_string()),
            environment: env::var("ROCKET_ENV").unwrap_or("development".to_string()),
        }
    }
}
