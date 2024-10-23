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
    cx: i32,
    cy: i32,
    radius: i32,
    glow_radius: i32,
    animation_delay: i32,
}

pub struct GrassPoint {
    x: i32,
    left_offset: i32,
    right_offset: i32,
    variation: i32,
}

impl GrassPoint {
    fn new(base_x: i32, index: i32) -> Self {
        let variation = ((index * 23) % 8) as i32;
        let offset_base = 5 + variation;

        Self {
            x: base_x,
            left_offset: if base_x > offset_base {
                base_x - offset_base
            } else {
                0
            },
            right_offset: base_x.saturating_add(offset_base),
            variation,
        }
    }
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate<'r> {
    pub dev_mode: bool,
    pub latest_notes: Vec<Note<'r>>,
    pub garden_paths: Vec<GardenPath<'r>>,
    pub forest_spirits: Vec<ForestSpirit>,
    pub grass_points: Vec<GrassPoint>,
    pub grass_points_offset1: Vec<GrassPoint>,
    pub grass_points_offset2: Vec<GrassPoint>,
}

#[get("/")]
pub fn index<'r>(config: &rocket::State<Config>) -> HomeTemplate<'r> {
    let grass_points: Vec<GrassPoint> = (0..80i32)
        .map(|i| {
            let base_x = i * 15;
            GrassPoint::new(base_x, i)
        })
        .collect();

    let grass_points_offset1: Vec<GrassPoint> = (0..80i32)
        .map(|i| {
            let base_x = i * 15 + 7;
            GrassPoint::new(base_x, i)
        })
        .collect();

    let grass_points_offset2: Vec<GrassPoint> = (0..80i32)
        .map(|i| {
            let base_x = i * 15 + 3;
            GrassPoint::new(base_x, i)
        })
        .collect();

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
        grass_points,
        grass_points_offset1,
        grass_points_offset2,
    }
}
