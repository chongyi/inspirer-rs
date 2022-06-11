use serde::{Deserialize, Serialize};
pub use crate::entity::contents::Model as ContentModel;
pub use crate::entity::content_entities::Model as ContentEntityModel;
#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct NewContent {
    #[serde(flatten)]
    pub meta: ContentMeta,
    pub entity: ContentEntity,
}

#[derive(Debug, Deserialize, Default, Serialize)]
#[serde(default)]
pub struct ContentMeta {
    pub title: String,
    pub keywords: String,
    pub description: String,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ContentEntity {
    Post(String),
    Page(String),
}

impl Default for ContentEntity {
    fn default() -> Self {
        ContentEntity::Post(String::new())
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Default)]
#[serde(default)]
pub struct GetListCondition {
    pub with_hidden: bool,
    pub with_unpublish: bool,
    pub without_page: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct Content {
    #[serde(flatten)]
    pub meta: ContentModel,
    pub entity: ContentEntity
}