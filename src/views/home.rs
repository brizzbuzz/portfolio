use askama::Template;
use rocket::get;

pub struct Name<'a> {
    first: &'a str,
    last: &'a str,
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate<'a> {
    pub name: Name<'a>,
    pub description: &'a str,
}

#[get("/")]
pub fn index() -> HomeTemplate<'static> {
    HomeTemplate {
        name: Name {
            first: "Ryan",
            last: "Brink",
        },
        description: "Occasionally I write things, more often I code things",
    }
}
