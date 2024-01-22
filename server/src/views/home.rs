use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct HelloTemplate<'a> {
    name: &'a str,
}

#[get("/")]
pub fn index() -> HelloTemplate<'static> {
    HelloTemplate { name: "Ryan" }
}
