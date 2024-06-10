use askama::Template;
use rocket::get;

#[derive(Template)]
#[template(path = "projects.html")]
pub struct ProjectsTemplate {}

#[get("/")]
pub fn index() -> ProjectsTemplate {
    ProjectsTemplate {}
}
