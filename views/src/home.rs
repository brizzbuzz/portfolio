use askama::Template;
use rocket::get;

#[derive(Template)]
#[template(path = "index.html")]
pub struct HomeTemplate<'a> {
    pub name: &'a str,
    pub description: &'a str,
}

#[get("/")]
pub fn index() -> HomeTemplate<'static> {
    HomeTemplate {
        name: "RYAN BRINK",
        description: "I’m Ryan, a software developer and all-around technochad based in New York City. Occasionally I write things, more often I code things. AI art enthusiast."
    }
}
