use chrono::NaiveDateTime;
use diesel;
use diesel::*;

use super::{GetDescription, ContentRelate};
use schema::content_articles;
use database::Conn;
use models::content::{UpdateContentEntity, ContentEntityDisplay};
use util::error::{ApplicationError as Error, database::map_database_error};

#[derive(Serialize, Queryable)]
pub struct ArticleDisplay {
    pub id: u32,
    pub content: String,
    pub name: Option<String>,
    pub views: u32,
    pub modified_at: Option<NaiveDateTime>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateArticle {
    pub content: String,
    pub name: Option<String>,
}

impl GetDescription for CreateArticle {
    fn description(&self) -> String {
        let sub = self.content.chars().into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
        let len = sub.len();
        if len > 240 {
            sub[..240].concat()
        } else {
            sub[..len].concat()
        }
    }
}

#[derive(Deserialize, Clone, Debug, AsChangeset)]
#[table_name = "content_articles"]
pub struct UpdateArticle {
    pub content: Option<String>,
    pub name: Option<String>,
}

#[derive(Insertable)]
#[table_name = "content_articles"]
pub struct NewArticle {
    pub content: String,
    pub name: Option<String>,
}

impl From<CreateArticle> for NewArticle {
    fn from(create: CreateArticle) -> Self {
        NewArticle {
            content: create.content.clone(),
            name: create.name.clone(),
        }
    }
}

pub struct Article;

impl ContentRelate for Article {
    fn find_by_id(connection: &Conn, entity_id: u32) -> Result<ContentEntityDisplay, Error> {
        use schema::content_articles::dsl::*;

        Ok(ContentEntityDisplay::Article(
            find_by_id!(
                connection => (
                    content_articles(
                        (id, content, name, views, modified_at)
                    ) # = entity_id => ArticleDisplay
                )
            )?
        ))
    }

    fn delete_by_content_id(connection: &Conn, cid: u32) -> bool {
        use schema::content_articles::dsl::*;

        let res = delete_by_id!(
            connection => (
                content_articles content_id = cid
            )
        );
        match res {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn update_by_id(connection: &Conn, entity_id: u32, update: UpdateContentEntity) -> Result<ContentEntityDisplay, Error> {
        use schema::content_articles::dsl::*;

        let r = (match update {
            UpdateContentEntity::Article(r) => Some(r),
            _ => None,
        }).unwrap();

        let count = update_by_id!(
            connection => (
                content_articles # = entity_id; <- &r
            )
        )?;

        Ok(ContentEntityDisplay::Article(
            find_by_id!(
                connection => (
                    content_articles(
                        (id, content, name, views, modified_at)
                    ) # = entity_id => ArticleDisplay
                )
            )?
        ))
    }
}