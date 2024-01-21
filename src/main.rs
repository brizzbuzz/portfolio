#[macro_use] extern crate rocket;
use askama::Template;
use crate::views::index::index;

pub mod views;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

//
// fn main() {
//     let hello = HelloTemplate { name: "world" };
//     println!("{}", hello.render().unwrap());
// }