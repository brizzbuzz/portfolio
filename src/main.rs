mod api;
mod views;

#[macro_use]
extern crate rocket;

use api::hot_reload;
use rocket::fs::{relative, FileServer};
use views::{home, posts, xcombinator};

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![home::index])
        .mount("/api", routes![hot_reload::index])
        .mount("/posts", routes![posts::index])
        .mount("/x-combinator", routes![xcombinator::index])
        .mount("/public/", FileServer::from(relative!("public")))
}
