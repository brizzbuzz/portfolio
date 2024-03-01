#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use views::{home, posts};

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![home::index])
        .mount("/posts", routes![posts::index])
        .mount("/public/", FileServer::from(relative!("public")))
}
