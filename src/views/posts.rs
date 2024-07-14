use askama::Template;
use rocket::get;

#[derive(Template)]
#[template(path = "posts.html")]
pub struct PostsTemplate {}

#[get("/")]
pub fn index() -> PostsTemplate {
    PostsTemplate {}
}
