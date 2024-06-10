use askama::Template;
use rocket::get;

#[derive(Template)]
#[template(path = "photos.html")]
pub struct PhotosTemplate {}

#[get("/")]
pub fn index() -> PhotosTemplate {
    PhotosTemplate {}
}
