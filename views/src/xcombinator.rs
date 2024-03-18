use askama::Template;
use rocket::get;

#[derive(Template)]
#[template(path = "xcombinator.html")]
pub struct XCombinatorTemplate {}

#[get("/")]
pub fn index() -> XCombinatorTemplate {
    XCombinatorTemplate {}
}
