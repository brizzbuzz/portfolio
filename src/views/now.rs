use crate::config::Config;
use askama_rocket::Template;
use rocket::get;

#[derive(Template)]
#[template(path = "now.html")]
pub struct NowTemplate {
    pub dev_mode: bool,
    pub last_updated: String,
}

#[get("/")]
pub fn now(config: &rocket::State<Config>) -> NowTemplate {
    NowTemplate {
        dev_mode: config.environment == "development",
        last_updated: "November 16, 2024".to_string(),
    }
}
