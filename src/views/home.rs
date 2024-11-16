use crate::config::Config;
use crate::models::forest_spirit::ForestSpirit;
use askama_rocket::Template;
use rocket::get;

pub struct Note<'a> {
    title: &'a str,
    excerpt: &'a str,
    date: &'a str, // TODO: use chrono
    path: &'a str,
}

pub struct GardenPath<'a> {
    href: &'a str,
    title: &'a str,
    description: &'a str,
    icon_path: &'a str,
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate<'r> {
    pub dev_mode: bool,
    pub latest_notes: Vec<Note<'r>>,
    pub garden_paths: Vec<GardenPath<'r>>,
    pub forest_spirits: Vec<ForestSpirit>,
}

#[get("/")]
pub fn index<'r>(config: &rocket::State<Config>) -> HomeTemplate<'r> {
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
        garden_paths: vec![
            GardenPath {
                href: "/now",
                title: "The Present Moment",
                description: "What I'm focused on right now",
                icon_path: "/public/images/the-all-encompassing-now.svg",
            },
            GardenPath {
                href: "/essays",
                title: "Essays & Thoughts",
                description: "Long-form writing on technology, art, and life",
                icon_path: "/public/images/flickering-lantern.svg",
            },
            GardenPath {
                href: "/projects",
                title: "Digital Projects",
                description: "Things I've built and am building",
                icon_path: "/public/images/magical-sappling.svg",
            },
            GardenPath {
                href: "/library",
                title: "Reading Notes",
                description: "Books, articles, and other interesting finds",
                icon_path: "/public/images/mystical-crystals.svg",
            },
        ],
        forest_spirits: ForestSpirit::default_spirits(),
    }
}
