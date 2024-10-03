use crate::config::Config;
use askama::Template;
use rocket::get;

#[derive(Template)]
#[template(path = "posts.html")]
pub struct PostsTemplate {
    pub dev_mode: bool,
}

#[get("/")]
pub fn index(config: &rocket::State<Config>) -> PostsTemplate {
    PostsTemplate {
        dev_mode: config.environment == "development",
    }
}
