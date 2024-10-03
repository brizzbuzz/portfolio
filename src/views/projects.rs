use askama::Template;
use crate::config::Config;
use rocket::get;

#[derive(Template)]
#[template(path = "projects.html")]
pub struct ProjectsTemplate {
    pub dev_mode: bool,
}

#[get("/")]
pub fn index(config: &rocket::State<Config>) -> ProjectsTemplate {
    ProjectsTemplate {
        dev_mode: config.environment == "development",
    }
}
