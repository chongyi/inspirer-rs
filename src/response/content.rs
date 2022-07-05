use chrono::{DateTime, Utc};
use inspirer_content::{
    model::content::{Content, ContentEntity, ContentModel},
    util::uuid::uuid_to_base62,
};
use serde::Serialize;

pub use inspirer_content::model::content::ContentConfig;

#[derive(Debug, Clone, Serialize)]
pub struct ContentBase {
    pub id: String,
    pub name: String,
    pub title: String,
    pub keywords: String,
    pub description: String,
    pub published_at: Option<DateTime<Utc>>,
}

impl From<ContentModel> for ContentBase {
    fn from(content_raw: ContentModel) -> Self {
        let id = uuid_to_base62(content_raw.id);
        ContentBase {
            id: id.clone(),
            name: content_raw.content_name.unwrap_or("".into()),
            title: content_raw.title,
            keywords: content_raw.keywords,
            description: content_raw.description,
            published_at: content_raw.published_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ContentWithEntity {
    #[serde(flatten)]
    pub base: ContentBase,
    pub entity: ContentEntity,
}

impl From<Content> for ContentWithEntity {
    fn from(Content { meta, entity }: Content) -> Self {
        ContentWithEntity {
            base: ContentBase::from(meta),
            entity,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ContentFull {
    #[serde(flatten)]
    pub base: ContentBase,
    pub content_type: u32,
    pub is_publish: bool,
    pub is_display: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<ContentModel> for ContentFull {
    fn from(content_raw: ContentModel) -> Self {
        ContentFull {
            content_type: content_raw.content_type as u32,
            is_display: content_raw.is_display,
            is_publish: content_raw.is_publish,
            created_at: content_raw.created_at,
            updated_at: content_raw.updated_at,
            base: content_raw.into(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ContentFullWithEntity {
    #[serde(flatten)]
    pub content: ContentFull,
    pub entity: ContentEntity,
}
