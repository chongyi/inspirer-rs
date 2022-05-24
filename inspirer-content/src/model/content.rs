use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct NewContent {
    #[serde(flatten)]
    pub meta: NewContentMeta,
    pub entity: ContentEntity,
}

#[derive(Debug, Deserialize, Default, Serialize)]
#[serde(default)]
pub struct NewContentMeta {
    pub id: Uuid,
    pub title: String,
    pub keywords: String,
    pub description: String,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum ContentEntity {
    Post(PostContent)
}

impl Default for ContentEntity {
    fn default() -> Self {
        ContentEntity::Post(PostContent::default())
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PostContent {
    pub content: String,
}