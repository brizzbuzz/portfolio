use crate::config::Config;
use crate::models::forest_spirit::ForestSpirit;
use askama_rocket::Template;
use rocket::get;

#[derive(Template)]
#[template(path = "now.html")]
pub struct NowTemplate {
    pub dev_mode: bool,
    pub last_updated: String,
    pub forest_spirits: Vec<ForestSpirit>, 
}

#[get("/")]
pub fn now(config: &rocket::State<Config>) -> NowTemplate {
    NowTemplate {
        dev_mode: config.environment == "development",
        last_updated: "November 16, 2024".to_string(),
        forest_spirits: ForestSpirit::default_spirits(), // Initialize forest spirits
    }
}
