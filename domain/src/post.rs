use uuid::Uuid;

pub struct Post {
    pub id: Option<Uuid>,
    pub title: String,
    pub body: String,
    pub published: bool,
}
