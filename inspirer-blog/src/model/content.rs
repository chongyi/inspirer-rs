use chrono::{DateTime, Utc};

pub struct NewContent<'a> {
    pub creator_id: u64,
    pub title: &'a str,
    pub keywords: &'a str,
    pub description: &'a str,
    pub content_entity_id: u64,
    pub is_display: bool,
}

pub struct NewContentEntity<'a> {
    pub creator_id: u64,
    pub is_draft: bool,
    pub title: &'a str,
    pub keywords: &'a str,
    pub description: &'a str,
    pub content: &'a str,
}

#[derive(sqlx::FromRow)]
pub struct ContentEntityFull {
    pub creator_id: u64,
    pub is_draft: bool,
    pub title: String,
    pub keywords: String,
    pub description: String,
    pub content: String,
    pub created_at: DateTime<Utc>
}

#[derive(sqlx::FromRow)]
pub struct ContentEntityBasic {
    pub creator_id: u64,
    pub is_draft: bool,
    pub title: String,
    pub keywords: String,
    pub description: String,
    pub created_at: DateTime<Utc>
}

pub struct NewContentEntityWithContent<'a> {
    pub content_id: u64,
    pub entity: NewContentEntity<'a>
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct ContentBasic {
    pub id: u64,
    pub creator_id: u64,
    pub creator_name: String,
    pub title: String,
    pub keywords: String,
    pub description: String,
    pub is_display: bool,
    pub is_published: bool,
    pub published_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 用于前端接口读取的内容数据
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct ContentForClient {
    pub id: u64,
    pub creator_id: u64,
    pub creator_name: String,
    pub title: String,
    pub keywords: String,
    pub description: String,
    pub content: String,
    pub published_at: DateTime<Utc>,
}

/// 用于前端接口读取的内容数据
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct ContentForClientBasic {
    pub id: u64,
    pub creator_id: u64,
    pub creator_name: String,
    pub title: String,
    pub keywords: String,
    pub description: String,
    pub published_at: DateTime<Utc>,
}

pub struct NewContentMeta<'a> {
    pub id: u64,
    pub content_entity_id: u64,
    pub title: &'a str,
    pub keywords: &'a str,
    pub description: &'a str,
}