use askama::Template;
use rocket::get;

struct PostMetadata {
    title: String,
    word_count: usize,
    publish_date: String,
    snippet: String,
}

#[derive(Template)]
#[template(path = "posts.html")]
pub struct PostsTemplate {
    posts: Vec<PostMetadata>,
}

#[get("/")]
pub fn index() -> PostsTemplate {
    let posts: Vec<PostMetadata> = vec![
        PostMetadata {
            title: "My First Article".to_string(),
            word_count: 100,
            publish_date: "2021-01-01".to_string(),
            snippet: "This is my first article".to_string(),
        },
        PostMetadata {
            title: "My Second Article".to_string(),
            word_count: 200,
            publish_date: "2021-01-02".to_string(),
            snippet: "This is my second article".to_string(),
        },
        PostMetadata {
            title: "My Third Article".to_string(),
            word_count: 300,
            publish_date: "2021-01-03".to_string(),
            snippet: "This is my third article".to_string(),
        },
    ];
    PostsTemplate { posts }
}
