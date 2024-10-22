use crate::config::Config;
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

pub struct ForestSpirit {
    cx: u16,
    cy: u16,
    radius: u8,
    glow_radius: u8,
    animation_delay: u16,
}

pub struct FloatingLeaf {
    y: u8,  // percentage from top of screen
    scale: f32,
    animation_delay: u16,
    variant: u8,
    color_class: String,
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate<'r> {
    pub dev_mode: bool,
    pub latest_notes: Vec<Note<'r>>,
    pub garden_paths: Vec<GardenPath<'r>>,
    pub forest_spirits: Vec<ForestSpirit>,
    pub floating_leaves: Vec<FloatingLeaf>,
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
        forest_spirits: vec![
            ForestSpirit {
                cx: 150,
                cy: 150,
                radius: 12,
                glow_radius: 8,
                animation_delay: 0,
            },
            ForestSpirit {
                cx: 850,
                cy: 250,
                radius: 15,
                glow_radius: 11,
                animation_delay: 2000,
            },
            ForestSpirit {
                cx: 450,
                cy: 750,
                radius: 10,
                glow_radius: 6,
                animation_delay: 1000,
            },
            ForestSpirit {
                cx: 750,
                cy: 550,
                radius: 14,
                glow_radius: 10,
                animation_delay: 3000,
            },
            ForestSpirit {
                cx: 250,
                cy: 450,
                radius: 11,
                glow_radius: 7,
                animation_delay: 2000,
            },
            ForestSpirit {
                cx: 650,
                cy: 350,
                radius: 13,
                glow_radius: 9,
                animation_delay: 1000,
            },
        ],
        floating_leaves: vec![
            // First wave
            FloatingLeaf {
                y: 15,
                scale: 0.9,
                animation_delay: 0,
                variant: 1,
                color_class: "fill-sage-200".to_string(),
            },
            FloatingLeaf {
                y: 35,
                scale: 0.8,
                animation_delay: 1500,
                variant: 2,
                color_class: "fill-terra-300".to_string(),
            },
            FloatingLeaf {
                y: 55,
                scale: 1.0,
                animation_delay: 3000,
                variant: 3,
                color_class: "fill-sage-300".to_string(),
            },
            // Second wave
            FloatingLeaf {
                y: 25,
                scale: 0.85,
                animation_delay: 4500,
                variant: 1,
                color_class: "fill-terra-200".to_string(),
            },
            FloatingLeaf {
                y: 45,
                scale: 0.95,
                animation_delay: 6000,
                variant: 2,
                color_class: "fill-sage-400".to_string(),
            },
            FloatingLeaf {
                y: 65,
                scale: 0.75,
                animation_delay: 7500,
                variant: 3,
                color_class: "fill-terra-300".to_string(),
            },
            // Additional waves with more varied positions
            FloatingLeaf {
                y: 20,
                scale: 0.9,
                animation_delay: 9000,
                variant: 1,
                color_class: "fill-sage-200".to_string(),
            },
            FloatingLeaf {
                y: 40,
                scale: 0.85,
                animation_delay: 10500,
                variant: 2,
                color_class: "fill-terra-200".to_string(),
            },
            FloatingLeaf {
                y: 60,
                scale: 0.95,
                animation_delay: 12000,
                variant: 3,
                color_class: "fill-sage-300".to_string(),
            },
        ],
    }
}
