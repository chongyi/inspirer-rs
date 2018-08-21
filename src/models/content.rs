use actix::{Message, Handler};
use diesel;
use diesel::*;
use chrono::NaiveDateTime;

use result::Result;
use database::{DatabaseExecutor, Conn, last_insert_id};
use message::{PaginatedListMessage, Pagination, UpdateByID};
use error::{Error, database::map_database_error};
use schema::contents;
use schema::contents::dsl as column;

#[derive(Deserialize, Insertable, Debug)]
#[table_name = "contents"]
pub struct NewContent {
    pub name: Option<String>,
    pub title: String,
    pub category_id: Option<u32>,
    pub keywords: String,
    pub description: String,
    pub sort: i16,
    pub content_type: u16,
    pub content: Option<String>,
    pub display: bool,
    pub published_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, AsChangeset, Debug)]
#[table_name = "contents"]
pub struct UpdateContent {
    pub name: Option<String>,
    pub title: Option<String>,
    pub category_id: Option<u32>,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub sort: Option<i16>,
    pub content_type: Option<u16>,
    pub content: Option<String>,
    pub display: Option<bool>,
    pub published_at: Option<NaiveDateTime>,
    pub modified_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct ContentDisplay {
    pub id: u32,
    pub name: Option<String>,
    pub title: String,
    pub category_id: Option<u32>,
    pub keywords: String,
    pub description: String,
    pub sort: i16,
    pub content_type: u16,
    pub display: bool,
    pub published_at: Option<NaiveDateTime>,
    pub modified_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct ContentFullDisplay {
    pub id: u32,
    pub name: Option<String>,
    pub title: String,
    pub category_id: Option<u32>,
    pub keywords: String,
    pub description: String,
    pub sort: i16,
    pub content_type: u16,
    pub content: Option<String>,
    pub display: bool,
    pub published_at: Option<NaiveDateTime>,
    pub modified_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

pub struct Content;

impl Content {
    pub const DISPLAY_COLUMNS: (
        column::id, column::name, column::title,
        column::category_id, column::keywords, column::description,
        column::sort, column::content_type, column::display,
        column::published_at, column::modified_at
    ) = (
        column::id, column::name, column::title,
        column::category_id, column::keywords, column::description,
        column::sort, column::content_type, column::display,
        column::published_at, column::modified_at
    );

    pub const DISPLAY_BASE_COLUMNS: (
        column::id, column::name, column::title,
        column::category_id,  column::sort, column::content_type, column::display
    ) = (
        column::id, column::name, column::title,
        column::category_id,  column::sort, column::content_type, column::display
    );

    pub fn create(connection: &Conn, data: NewContent) -> Result<u32> {
        use schema::contents::dsl::*;

        diesel::insert_into(contents)
            .values(&data)
            .execute(connection)
            .map_err(map_database_error(Some("contents")))?;

        let generated_id: u64 = diesel::select(last_insert_id)
            .first(connection)
            .map_err(map_database_error(Some("contents")))?;

        Ok(generated_id as u32)
    }

    pub fn find_by_id(connection: &Conn, target: u32) -> Result<ContentFullDisplay> {
        use schema::contents::dsl::*;

        find_by_id!(connection => (
            contents # = target => ContentFullDisplay
        ))
    }

    pub fn find_by_name(connection: &Conn, target: String) -> Result<ContentFullDisplay> {
        use schema::contents::dsl::*;

        find_by_id!(connection => (
            contents name = target => ContentFullDisplay
        ))
    }

    pub fn update(connection: &Conn, target: u32, data: UpdateContent) -> Result<Option<ContentFullDisplay>> {
        use schema::contents::dsl::*;

        let count = update_by_id!(connection => (
            contents # = target; <- &data
        ))?;

        if count > 0 {
            Ok(Self::find_by_id(connection, target).ok())
        } else {
            Ok(None)
        }
    }

    pub fn get_list(connection: &Conn, c: Pagination<GetContents>) -> Result<PaginatedListMessage<ContentDisplay>> {
        use schema::contents::dsl::*;

        let paginator = paginator!(connection, Self::DISPLAY_COLUMNS, c, ContentDisplay, {
            let mut query = contents.into_boxed();
            if let Some(filter) = c.clone().filter {
                if let Some(v) = filter.search {
                    query = query.filter(name.like(format!("%{}%", &v)).or(title.like(format!("%{}%", &v))));
                }

                if let Some(v) = filter.display {
                    query = query.filter(display.eq(v));
                }

            }

            query.order((sort.desc(), published_at.desc(), id.desc(), created_at.desc()))
        });

        paginator()
    }
}

#[derive(Clone, Debug)]
pub struct GetContents {
    pub search: Option<String>,
    pub category: Option<String>,
    pub display: Option<bool>,
}

impl Default for GetContents {
    fn default() -> Self {
        GetContents {
            search: None,
            category: None,
            display: Some(true)
        }
    }
}

impl Message for Pagination<GetContents> {
    type Result = Result<PaginatedListMessage<ContentDisplay>>;
}

impl Handler<Pagination<GetContents>> for DatabaseExecutor {
    type Result = <Pagination<GetContents> as Message>::Result;

    fn handle(&mut self, msg: Pagination<GetContents>, ctx: &mut Self::Context) -> Self::Result {
        Content::get_list(&self.connection()?, msg)
    }
}

pub enum FindContent {
    ById(u32),
    ByName(String),
}

impl Message for FindContent {
    type Result = Result<ContentFullDisplay>;
}

impl Handler<FindContent> for DatabaseExecutor {
    type Result = <FindContent as Message>::Result;

    fn handle(&mut self, msg: FindContent, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            FindContent::ByName(name) => Content::find_by_name(&self.connection()?, name),
            FindContent::ById(id) => Content::find_by_id(&self.connection()?, id)
        }
    }
}