pub use crate::entity::content_entities::Model as ContentEntityModel;
pub use crate::entity::contents::Model as ContentModel;
use crate::model::user::UserModel;
use serde::{Deserialize, Serialize};

use super::Order;
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

#[derive(Debug, Deserialize, Default, Serialize)]
#[serde(default)]
pub struct UpdateContentMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Default, Serialize)]
#[serde(default)]
pub struct UpdateContent {
    #[serde(flatten)]
    pub meta: UpdateContentMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<ContentEntity>,
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
    pub list_deleted: bool,
    pub sort: Vec<Order<SortField>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortField {
    PublishedAt,
    CreatedAt,
    DeletedAt,
}

impl Into<crate::entity::contents::Column> for SortField {
    fn into(self) -> crate::entity::contents::Column {
        match self {
            SortField::CreatedAt => crate::entity::contents::Column::CreatedAt,
            SortField::PublishedAt => crate::entity::contents::Column::PublishedAt,
            SortField::DeletedAt => crate::entity::contents::Column::DeletedAt,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Content {
    #[serde(flatten)]
    pub meta: ContentModel,
    pub entity: ContentEntity,
}

impl From<NewContent> for UpdateContent {
    fn from(new_content: NewContent) -> Self {
        let NewContent { meta, entity } = new_content;
        UpdateContent {
            meta: UpdateContentMeta {
                title: Some(meta.title),
                keywords: Some(meta.keywords),
                description: Some(meta.description),
                name: meta.name,
            },
            entity: Some(entity),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ContentConfig {
    pub content_support_type: &'static [&'static str],
}