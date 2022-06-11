use chrono::{DateTime, Utc};
use inspirer_content::{model::content::{ContentEntity, ContentModel}, util::uuid::uuid_to_base62};
use serde::Serialize;

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
            name: content_raw
                .content_name
                .unwrap_or(id),
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
