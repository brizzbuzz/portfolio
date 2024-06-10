use askama::Template;
use rocket::get;

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate {}

#[get("/")]
pub fn index() -> AboutTemplate {
    AboutTemplate {}
}
