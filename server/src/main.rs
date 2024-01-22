#[macro_use]
extern crate rocket;
use crate::views::index::index;
use rocket::fs::{relative, FileServer};

pub mod views;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/public/", FileServer::from(relative!("public")))
}
