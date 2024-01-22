#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, relative};
use crate::views::index::index;

pub mod views;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/public/", FileServer::from(relative!("public")))
}
