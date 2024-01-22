#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};

pub mod views;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![views::home::index])
        .mount("/articles", routes![views::articles::index])
        .mount("/public/", FileServer::from(relative!("public")))
}
