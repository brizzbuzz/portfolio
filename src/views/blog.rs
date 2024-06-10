use askama::Template;
use rocket::get;

#[derive(Template)]
#[template(path = "blog.html")]
pub struct BlogTemplate {}

#[get("/")]
pub fn index() -> BlogTemplate {
    BlogTemplate {}
}
