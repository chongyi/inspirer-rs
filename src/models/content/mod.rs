pub mod article;

use chrono::NaiveDateTime;
use diesel;
use diesel::*;
use actix::*;
use actix_web::*;

use database::{DatabaseExecutor, Conn, last_insert_id};
use util::message::CreatedObjectIdMessage;
use util::error::ApplicationError as Error;
use self::article::{ArticleDisplay, CreateArticle, NewArticle};
use schema::contents;

pub trait GetDescription {
    fn description(&self) -> String;
}

pub enum ContentEntityDisplay {
    Article(ArticleDisplay),
}

#[derive(Clone, Deserialize)]
#[serde(tag = "entity_type", content = "body", rename_all = "snake_case")]
pub enum CreateContentEntity {
    Article(CreateArticle),
}

impl From<CreateContentEntity> for u16 {
    fn from(create: CreateContentEntity) -> Self {
        match create {
            CreateContentEntity::Article(_) => Content::CONTENT_TYPE_ARTICLE,
        }
    }
}

pub struct ContentDisplay {
    pub id: u32,
    pub creator_id: u32,
    pub title: String,
    pub keywords: String,
    pub description: String,
    pub sort: u16,
    pub category_id: Option<u32>,
    pub entity: ContentEntityDisplay,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct ContentBase {
    pub id: u32,
    pub creator_id: u32,
    pub title: String,
    pub sort: u16,
    pub category_id: Option<u32>,
    pub content_id: u32,
    pub content_type: u16,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct CreateContent {
    pub creator_id: u32,
    pub title: String,
    pub category_id: Option<u32>,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub sort: Option<u16>,
    pub display: Option<bool>,
    pub entity: CreateContentEntity,
}


#[derive(Insertable)]
#[table_name = "contents"]
pub struct NewContent {
    pub creator_id: u32,
    pub title: String,
    pub category_id: Option<u32>,
    pub keywords: String,
    pub description: String,
    pub sort: Option<u16>,
    pub display: Option<bool>,
    pub content_type: u16,
    pub content_id: u32,
}

pub struct Content;

impl Content {
    pub const CONTENT_TYPE_ARTICLE: u16 = 1;

    pub fn create_content(connection: &Conn, create: CreateContent) -> Result<u64, Error> {
        let (id, description) = match create.entity.clone() {
            CreateContentEntity::Article(article) => {
                use schema::content_articles::dsl::*;
                let new_article: NewArticle = article.clone().into();
                diesel::insert_into(content_articles)
                    .values(new_article)
                    .execute(connection)
                    .map_err(map_database_error!("content_articles"))?;

                let generated_id: u64 = last_insert_id!(connection, "content_articles");

                (generated_id, article.description())
            }
        };

        let new_content = NewContent {
            creator_id: create.creator_id,
            title: create.title,
            category_id: create.category_id,
            keywords: create.keywords.unwrap_or(String::from("")),
            description: create.description.unwrap_or(String::from("")),
            sort: create.sort,
            display: create.display,
            content_id: id as u32,
            content_type: create.entity.into(),
        };

        {
            use schema::contents::dsl::*;
            diesel::insert_into(contents)
                .values(new_content)
                .execute(connection)
                .map_err(map_database_error!("contents"))?;

            let generated_id: u64 = last_insert_id!(connection, "contents");

            Ok(generated_id)
        }
    }
}

impl Message for CreateContent {
    type Result = Result<u64, Error>;
}

impl Handler<CreateContent> for DatabaseExecutor {
    type Result = Result<u64, Error>;

    fn handle(&mut self, msg: CreateContent, _: &mut Self::Context) -> Self::Result {
        Content::create_content(&self.connection()?, msg)
    }
}