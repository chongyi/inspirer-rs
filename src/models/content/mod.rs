pub mod article;

use chrono::NaiveDateTime;
use diesel;
use diesel::*;
use actix::*;
use actix_web::*;

use database::{DatabaseExecutor, Conn, last_insert_id};
use util::message::{CreatedObjectIdMessage, PaginatedListMessage, Pagination, UpdateByID};
use util::error::{ApplicationError as Error, database::map_database_error};
use self::article::{ArticleDisplay, CreateArticle, NewArticle, Article, UpdateArticle};
use schema::contents;

pub trait GetDescription {
    fn description(&self) -> String;
}

pub trait ContentRelate {
    fn find_by_id(connection: &Conn, entity_id: u32) -> Result<ContentEntityDisplay, Error>;
    fn delete_by_content_id(connection: &Conn, content_id: u32) -> bool;
    fn update_by_id(connection: &Conn, entity_id: u32, update: UpdateContentEntity) -> Result<ContentEntityDisplay, Error>;
}

#[derive(Serialize)]
#[serde(tag = "entity_type", content = "body")]
pub enum ContentEntityDisplay {
    #[serde(rename = "article")]
    Article(ArticleDisplay),
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(tag = "entity_type", content = "body")]
pub enum CreateContentEntity {
    #[serde(rename = "article")]
    Article(CreateArticle),
}

impl From<CreateContentEntity> for u16 {
    fn from(create: CreateContentEntity) -> Self {
        match create {
            CreateContentEntity::Article(_) => Content::CONTENT_TYPE_ARTICLE,
        }
    }
}

#[derive(Serialize, Queryable)]
pub struct ContentDisplay {
    pub id: u32,
    pub creator_id: u32,
    pub title: String,
    pub keywords: String,
    pub description: String,
    pub sort: u16,
    pub display: bool,
    pub category_id: Option<u32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize)]
pub struct ContentFullDisplay {
    pub id: u32,
    pub creator_id: u32,
    pub title: String,
    pub keywords: String,
    pub description: String,
    pub sort: u16,
    pub category_id: Option<u32>,
    pub display: bool,
    pub entity: ContentEntityDisplay,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Queryable)]
pub struct ContentRelateBase {
    pub id: u32,
    pub content_id: u32,
    pub content_type: u16,
}

#[derive(Debug, Clone, Serialize, Queryable)]
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

#[derive(Debug, Clone, Serialize, Queryable)]
pub struct ContentFull {
    pub id: u32,
    pub creator_id: u32,
    pub title: String,
    pub category_id: Option<u32>,
    pub keywords: String,
    pub description: String,
    pub sort: u16,
    pub display: bool,
    pub content_type: u16,
    pub content_id: u32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateContent {
    pub creator_id: Option<u32>,
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

type PaginatedContentList = Result<PaginatedListMessage<ContentBase>, Error>;
type DisplayContentDetail = Result<ContentFullDisplay, Error>;

pub struct Content;

impl Content {
    pub const CONTENT_TYPE_ARTICLE: u16 = 1;

    pub fn create_content(connection: &Conn, create: CreateContent) -> Result<u64, Error> {
        let (id, description, refresh_id) = match create.entity.clone() {
            CreateContentEntity::Article(article) => {
                use schema::content_articles::dsl::*;
                let new_article: NewArticle = article.clone().into();
                diesel::insert_into(content_articles)
                    .values(new_article)
                    .execute(connection)
                    .map_err(map_database_error("content_articles"))?;

                let generated_id = last_insert_id!(connection, "content_articles") as u32;

                (generated_id, article.description(), move |cid: u32, conn: &Conn| -> Result<_, Error> {
                    Ok(
                        diesel::update(content_articles)
                            .set(content_id.eq(cid))
                            .filter(id.eq(generated_id))
                            .execute(conn)
                            .map_err(map_database_error("content_articles"))?
                    )
                })
            }
        };

        let new_content = NewContent {
            creator_id: create.creator_id.unwrap(),
            title: create.title,
            category_id: create.category_id,
            keywords: create.keywords.unwrap_or(String::from("")),
            description: create.description.unwrap_or(String::from("")),
            sort: create.sort,
            display: create.display,
            content_id: id,
            content_type: create.entity.into(),
        };

        {
            use schema::contents::dsl::*;
            diesel::insert_into(contents)
                .values(new_content)
                .execute(connection)
                .map_err(map_database_error("contents"))?;

            let generated_id: u64 = last_insert_id!(connection, "contents");
            refresh_id(generated_id as u32, connection)?;

            Ok(generated_id)
        }
    }

    pub fn get_content_list(connection: &Conn, c: Pagination<GetContentList>) -> PaginatedContentList {
        use schema::contents::dsl::*;

        let paginator = paginator!(connection, (id, creator_id, title, sort, category_id, content_id, content_type, created_at, updated_at), c, ContentBase, {
            let mut query = contents.into_boxed();
            if let Some(filter) = c.clone().filter {
                if let Some(v) = filter.search {
                    query = query.filter(
                            title.like(format!("%{}%", v))
                                .or(keywords.like(format!("%{}%", v)))
                                .or(description.like(format!("%{}%", v)))
                        );
                }

                if let Some(t) = filter.content_type {
                    query = query.filter(content_type.eq(t));
                }
            }

            query.order((sort.desc(), created_at.desc(), id.desc()))
        });

        paginator()
    }

    pub fn find_content_by_id(connection: &Conn, cid: u32) -> Result<ContentFullDisplay, Error> {
        use schema::contents::dsl::*;

        let content: ContentFull = find_by_id!(
            connection => (
                contents # = cid => ContentFull
            )
        )?;

        let entity = match content.content_type {
            Self::CONTENT_TYPE_ARTICLE => Article::find_by_id(connection, content.content_id)?,
            _ => return Err(Error::SysLogicArgumentError()),
        };

        Ok(
            ContentFullDisplay {
                id: content.id,
                title: content.title.clone(),
                creator_id: content.creator_id,
                category_id: content.category_id,
                keywords: content.keywords.clone(),
                description: content.description.clone(),
                sort: content.sort,
                display: content.display,
                entity,
                created_at: content.created_at,
                updated_at: content.updated_at,
            }
        )
    }

    pub fn delete_content(connection: &Conn, cid: u32) -> Result<u32, Error> {
        use schema::contents::dsl::*;

        let target = contents.select((id, content_id, content_type))
            .filter(id.eq(cid))
            .first::<ContentRelateBase>(connection)
            .optional()
            .map_err(map_database_error("contents"))?;

        if let Some(t) = target {
            match t.content_type {
                1 => Article::delete_by_content_id(connection, cid),
                _ => true,
            };
        }

        let res = delete_by_id!(
            connection => (
                contents # = cid
            )
        )? as u32;

        Ok(res)
    }

    pub fn update_content(connection: &Conn, cid: u32, update: PreUpdateContent) -> DisplayContentDetail {
        let target: ContentRelateBase = {
            use schema::contents::dsl::*;

            contents.select((id, content_id, content_type))
                .filter(id.eq(cid))
                .first::<ContentRelateBase>(connection)
                .map_err(map_database_error("contents"))?
        };

        let entity_matches = update.entity.clone();
        let update_content = UpdateContent::from(update);
        let updated_entity = match entity_matches {
            Some(entity) => {
                match entity {
                    UpdateContentEntity::Article(_) => Article::update_by_id(connection, target.content_id, entity.clone())
                }
            }
            None => Err(Error::SysLogicArgumentError()),
        }?;

        let updated_content: ContentDisplay = {
            use schema::contents::dsl::*;
            update_by_id!(
                connection => (
                    contents # = cid; <- &update_content
                )
            )?;
            find_by_id!(
                connection => (
                    contents(
                        (id, creator_id, title, keywords, description, sort, display, category_id, created_at, updated_at)
                    ) # = cid => ContentDisplay
                )
            )?
        };

        Ok(
            ContentFullDisplay {
                id: updated_content.id,
                title: updated_content.title,
                creator_id: updated_content.creator_id,
                keywords: updated_content.keywords,
                description: updated_content.description,
                sort: updated_content.sort,
                display: updated_content.display,
                category_id: updated_content.category_id,
                created_at: updated_content.created_at,
                updated_at: updated_content.updated_at,
                entity: updated_entity,
            }
        )
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

#[derive(Clone, Debug, Deserialize)]
pub struct GetContentList {
    pub search: Option<String>,
    pub content_type: Option<u16>,
}

impl Message for Pagination<GetContentList> {
    type Result = PaginatedContentList;
}

impl Handler<Pagination<GetContentList>> for DatabaseExecutor {
    type Result = PaginatedContentList;

    fn handle(&mut self, condition: Pagination<GetContentList>, _: &mut Self::Context) -> Self::Result {
        Content::get_content_list(&self.connection()?, condition)
    }
}

#[derive(Clone, Deserialize, Debug)]
#[serde(tag = "entity_type", content = "body")]
pub enum UpdateContentEntity {
    #[serde(rename = "article")]
    Article(UpdateArticle),
}

#[derive(Deserialize, Clone)]
pub struct PreUpdateContent {
    pub title: Option<String>,
    pub category_id: Option<u32>,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub sort: Option<u16>,
    pub display: Option<bool>,
    pub entity: Option<UpdateContentEntity>,
}

#[derive(AsChangeset)]
#[table_name = "contents"]
pub struct UpdateContent {
    pub title: Option<String>,
    pub category_id: Option<u32>,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub sort: Option<u16>,
    pub display: Option<bool>,
}

impl From<PreUpdateContent> for UpdateContent {
    fn from(origin: PreUpdateContent) -> Self {
        UpdateContent {
            title: origin.title.clone(),
            category_id: origin.category_id,
            keywords: origin.keywords.clone(),
            description: origin.description.clone(),
            sort: origin.sort,
            display: origin.display,
        }
    }
}

impl Message for UpdateByID<PreUpdateContent> {
    type Result = DisplayContentDetail;
}

impl Handler<UpdateByID<PreUpdateContent>> for DatabaseExecutor {
    type Result = DisplayContentDetail;

    fn handle(&mut self, update: UpdateByID<PreUpdateContent>, _: &mut Self::Context) -> Self::Result {
        Content::update_content(&self.connection()?, update.id, update.update)
    }
}

pub struct DeleteContent(pub u32);

impl Message for DeleteContent {
    type Result = Result<u32, Error>;
}

impl Handler<DeleteContent> for DatabaseExecutor {
    type Result = Result<u32, Error>;

    fn handle(&mut self, finder: DeleteContent, _: &mut Self::Context) -> Self::Result {
        Content::delete_content(&self.connection()?, finder.0)
    }
}

pub struct FindContent(pub u32);

impl Message for FindContent {
    type Result = Result<ContentFullDisplay, Error>;
}

impl Handler<FindContent> for DatabaseExecutor {
    type Result = Result<ContentFullDisplay, Error>;

    fn handle(&mut self, finder: FindContent, _: &mut Self::Context) -> Self::Result {
        Content::find_content_by_id(&self.connection()?, finder.0)
    }
}