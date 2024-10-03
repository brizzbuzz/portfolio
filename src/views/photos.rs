use crate::config::Config;
use askama::Template;
use rocket::get;

#[derive(Template)]
#[template(path = "photos.html")]
pub struct PhotosTemplate {
    pub dev_mode: bool,
}

#[get("/")]
pub fn index(config: &rocket::State<Config>) -> PhotosTemplate {
    PhotosTemplate {
        dev_mode: config.environment == "development",
    }
}
