use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct HelloTemplate<'a> {
    name: &'a str,
}

#[derive(Responder)]
struct HelloResponder<'a> {
    template: HelloTemplate<'a>,
}

#[get("/")]
pub fn index() -> HelloTemplate<'static> {
    HelloTemplate { name: "Ryan" }
}
