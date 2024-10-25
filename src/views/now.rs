use crate::config::Config;
use askama_rocket::Template;
use rocket::get;

pub struct Priority<'a> {
    title: &'a str,
    description: &'a str,
    progress: u8, // 0-100
    category: &'a str, // "work", "personal", "learning", etc.
}

pub struct LifeSection<'a> {
    title: &'a str,
    content: &'a str,
}

#[derive(Template)]
#[template(path = "now.html")]
pub struct NowTemplate<'r> {
    pub dev_mode: bool,
    pub current_priorities: Vec<Priority<'r>>,
    pub life_sections: Vec<LifeSection<'r>>,
    pub last_updated: &'r str,
}

#[get("/")]
pub fn now<'r>(config: &rocket::State<Config>) -> NowTemplate<'r> {
    NowTemplate {
        dev_mode: config.environment == "development",
        current_priorities: vec![
            Priority {
                title: "Building My Digital Garden",
                description: "Creating a thoughtful, interconnected space for my writing and projects. Currently focusing on the core infrastructure and visual design.",
                progress: 65,
                category: "work",
            },
            Priority {
                title: "Deep Diving into Rust",
                description: "Mastering Rust's ownership model and exploring async programming patterns for web development.",
                progress: 40,
                category: "learning",
            },
            Priority {
                title: "Daily Writing Practice",
                description: "Writing daily, focusing on technical topics and personal reflections. Building a consistent publishing rhythm.",
                progress: 30,
                category: "personal",
            },
        ],
        life_sections: vec![
            LifeSection {
                title: "Reading",
                content: "Currently reading 'The Practice of Programming' by Kernighan and Pike, and 'Meditations' by Marcus Aurelius.",
            },
            LifeSection {
                title: "Location",
                content: "Based in [Your Location], focusing on building a calm and productive environment for deep work.",
            },
        ],
        last_updated: "October 20, 2024",
    }
}
