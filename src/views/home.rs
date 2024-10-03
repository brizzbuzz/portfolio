use askama::Template;
use crate::config::Config;
use rocket::get;

pub struct Name<'a> {
    first: &'a str,
    last: &'a str,
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate<'a> {
    pub dev_mode: bool,
    pub name: Name<'a>,
    pub description: &'a str,
}

#[get("/")]
pub fn index(config: &rocket::State<Config>) -> HomeTemplate<'static> {
    HomeTemplate {
        dev_mode: config.environment == "development",
        name: Name {
            first: "Ryan",
            last: "Brink",
        },
        description: "Occasionally I write things, more often I code things",
    }
}
