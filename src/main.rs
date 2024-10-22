mod config;
mod views;

use config::Config;
use rocket::response::stream::{Event, EventStream};
use std::time::Duration;
use tokio::time::interval;

#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;

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
    rocket::build()
        .manage(Config::build())
        .mount("/", routes![views::home::index, sse_events])
        .mount("/posts", routes![views::posts::index])
        .mount("/photos", routes![views::photos::index])
        .mount("/projects", routes![views::projects::index])
        .mount("/public/", FileServer::from(Config::build().asset_path))
}
