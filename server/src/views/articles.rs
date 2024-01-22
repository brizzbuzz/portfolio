use askama::Template;

struct ArticleMetadata {
    title: String,
    word_count: usize,
    publish_date: String,
    snippet: String,
}

#[derive(Template)]
#[template(path = "articles.html")]
pub struct ArticlesTemplate {
    articles: Vec<ArticleMetadata>,
}

#[get("/")]
pub fn index() -> ArticlesTemplate {
    let articles: Vec<ArticleMetadata> = vec![
        ArticleMetadata {
            title: "My First Article".to_string(),
            word_count: 100,
            publish_date: "2021-01-01".to_string(),
            snippet: "This is my first article".to_string(),
        },
        ArticleMetadata {
            title: "My Second Article".to_string(),
            word_count: 200,
            publish_date: "2021-01-02".to_string(),
            snippet: "This is my second article".to_string(),
        },
        ArticleMetadata {
            title: "My Third Article".to_string(),
            word_count: 300,
            publish_date: "2021-01-03".to_string(),
            snippet: "This is my third article".to_string(),
        },
    ];
    ArticlesTemplate { articles }
}
