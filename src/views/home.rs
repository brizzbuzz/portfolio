use crate::config::Config;
use askama::Template;
use rocket::get;

pub struct Note<'a> {
    title: &'a str,
    excerpt: &'a str,
    date: &'a str, // TODO: use chrono
    path: &'a str,
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate<'a> {
    pub dev_mode: bool,
    pub latest_notes: Vec<Note<'a>>
}

#[get("/")]
pub fn index(config: &rocket::State<Config>) -> HomeTemplate<'static> {
    HomeTemplate {
        dev_mode: config.environment == "development",
        latest_notes: vec![
            Note {
                title: "Hello, world!",
                excerpt: "This is a test note.",
                date: "2023-01-01",
                path: "/notes/hello-world",
            },
            Note {
                title: "Goodbye, world!",
                excerpt: "This is a test note.",
                date: "2023-01-01",
                path: "/notes/hello-world",
            },
        ],
    }
}
