mod views;
use std::env;

#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};

struct Config {
    asset_path: String,
}

impl Config {
    fn build() -> Self {
        Config {
            asset_path: env::var("ASSET_PATH").unwrap_or(relative!("public").to_string()),
        }
    }
}

#[launch]
async fn rocket() -> _ {
    let config = Config::build();
    rocket::build()
        .mount("/", routes![views::home::index])
        .mount("/posts", routes![views::posts::index])
        .mount("/photos", routes![views::photos::index])
        .mount("/projects", routes![views::projects::index])
        .mount("/public/", FileServer::from(config.asset_path))
}
