//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;
use serde::Serialize;

use crate::enumerate::content::ContentType;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "contents")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub owner_id: Uuid,
    pub authors: Json,
    #[sea_orm(unique)]
    pub content_name: Option<String>,
    pub content_type: ContentType,
    pub title: String,
    pub keywords: String,
    pub description: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub is_publish: bool,
    pub is_display: bool,
    pub published_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::content_entities::Entity")]
    Entity,
    #[sea_orm(belongs_to = "super::users::Entity", from = "Column::OwnerId", to = "super::users::Column::Id")]
    Owner,
}

impl Related<super::content_entities::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Entity.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Owner.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
