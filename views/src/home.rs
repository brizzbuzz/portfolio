use askama::Template;
use rocket::get;

#[derive(Template)]
#[template(path = "index.html")]
pub struct HelloTemplate<'a> {
    pub name: &'a str,
}

#[get("/")]
pub fn index() -> HelloTemplate<'static> {
    HelloTemplate { name: "Ryan Brink" }
}
