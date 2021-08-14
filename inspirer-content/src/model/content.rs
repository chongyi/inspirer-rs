use chrono::{DateTime, Utc};
use sqlx::{Executor, MySql};

use inspirer_query_ext::dao::{DAO, ReadDAO};

pub struct NewContent {
    pub author_id: u64,
}

pub struct NewContentEntity<'a> {
    pub author_id: u64,
    pub previous_id: u64,
    pub is_draft: bool,
    pub content_id: u64,
    pub entity: ContentEntityWritable<'a>,
}

pub struct UpdateContentEntity<'a> {
    pub id: u64,
    pub is_draft: bool,
    pub content_id: u64,
    pub entity: ContentEntityWritable<'a>,
}

pub struct ContentEntityWritable<'a> {
    pub title: &'a str,
    pub keywords: &'a str,
    pub description: &'a str,
    pub content: &'a str,
}

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct ContentEntity {
    pub is_draft: bool,
    pub title: String,
    pub keywords: String,
    pub description: String,
    pub content: String,
}

#[derive(sqlx::FromRow)]
pub struct ContentEntityFull {
    pub author_id: u64,
    pub is_draft: bool,
    pub title: String,
    pub keywords: String,
    pub description: String,
    pub content: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
pub struct ContentEntityMeta {
    pub id: u64,
    pub author_id: u64,
    pub is_draft: bool,
    pub previous_id: u64,
}

pub struct ContentStatusWritable {
    pub is_display: bool,
    pub is_published: bool,
}

pub struct BindContentToContentEntity {
    pub content_id: u64,
    pub content_entity_id: u64,
}


pub struct GetLatestContentEntity {
    pub content_id: u64,
    pub is_draft: bool,
}

pub struct DeleteContent(pub u64);

pub struct DeleteContentEntityByContentId(pub u64);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentWithEntity {
    #[serde(flatten)]
    pub content: ContentBasic,
    #[serde(flatten)]
    pub entity: ContentEntity,
}

#[derive(sqlx::FromRow, Clone, Debug, Serialize, Deserialize)]
pub struct ContentBasic {
    pub id: u64,
    pub author_id: u64,
    #[serde(skip_serializing)]
    pub is_display: bool,
    #[serde(skip_serializing)]
    pub is_deleted: bool,
    #[serde(skip_serializing)]
    pub is_published: bool,
    #[serde(skip_serializing)]
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: DateTime<Utc>,
}

pub struct ContentId (pub u64);