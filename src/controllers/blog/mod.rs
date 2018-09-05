use chrono::NaiveDateTime;

pub mod index;
pub mod content;
pub mod push_message;
pub mod rss;

#[derive(Serialize)]
pub struct Content {
    pub id: u32,
    pub name: Option<String>,
    pub title: String,
    pub description: String,
    pub published_at_o: Option<NaiveDateTime>,
    pub published_at: Option<String>,
}

#[derive(Serialize)]
pub struct PushMessage {
    pub id: u32,
    pub content: String,
    pub created_at: String,
    pub created_at_o: NaiveDateTime,
}