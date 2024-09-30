mod views;
use std::env;
use rocket::response::stream::{Event, EventStream};
use std::time::Duration;
use tokio::time::interval;

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

#[get("/sse")]
fn sse_events() -> EventStream![] {
    EventStream! {
        let mut interval = interval(Duration::from_secs(1));
        loop {
            yield Event::data("ping");
            interval.tick().await;
        }
    }
}

#[launch]
async fn rocket() -> _ {
    let config = Config::build();
    rocket::build()
        .mount("/", routes![views::home::index, sse_events])
        .mount("/posts", routes![views::posts::index])
        .mount("/photos", routes![views::photos::index])
        .mount("/projects", routes![views::projects::index])
        .mount("/public/", FileServer::from(config.asset_path))
}
