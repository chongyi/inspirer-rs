pub mod article;

use chrono::NaiveDateTime;

use database::{DatabaseExecutor, Conn, last_insert_id};
use self::article::ArticleDisplay;

pub enum ContentEntityDisplay {
    Article(ArticleDisplay),
}

pub struct ContentFullDisplay {
    pub id: u32,
    pub creator_id: u32,
    pub title: String,
    pub keywords: String,
    pub description: String,
    pub sort: u16,
    pub category_id: u32,
    pub entity: ContentEntityDisplay,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct ContentBase {
    pub id: u32,
    pub creator_id: u32,
    pub title: String,
    pub sort: u16,
    pub category_id: u32,
    pub content_id: u32,
    pub content_type: u16,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct Content;

impl Content {
    pub fn find_content(connection: &Conn, id: u32) {

    }
}