mod views;
use std::env;

#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use views::{home, posts, xcombinator};

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
        .mount("/", routes![home::index])
        .mount("/posts", routes![posts::index])
        .mount("/x-combinator", routes![xcombinator::index])
        .mount("/public/", FileServer::from(config.asset_path))
}
