use chrono::NaiveDateTime;

pub struct ArticleDisplay {
    pub id: u32,
    pub content: String,
    pub name: Option<String>,
    pub views: u32,
    pub modified_at: Option<NaiveDateTime>,
}