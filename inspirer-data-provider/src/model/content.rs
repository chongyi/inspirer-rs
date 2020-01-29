use crate::schema::contents;
use chrono::prelude::*;

#[allow(non_upper_case_globals)]
pub const content_base_columns: (
    contents::id,
    contents::uuid,
    contents::creator_uuid,
    contents::title,
    contents::content_type,
    contents::display,
    contents::published,
    contents::created_at,
    contents::updated_at,
) = (
    contents::id,
    contents::uuid,
    contents::creator_uuid,
    contents::title,
    contents::content_type,
    contents::display,
    contents::published,
    contents::created_at,
    contents::updated_at,
);

#[derive(Queryable, Debug, Clone, PartialEq, Serialize)]
pub struct ContentBase {
    pub id: i64,
    pub uuid: String,
    pub creator_uuid: String,
    pub title: Option<String>,
    pub content_type: i16,
    pub display: bool,
    pub published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug, Clone, PartialEq, Serialize)]
pub struct ContentFull {
    pub id: i64,
    pub uuid: String,
    pub version: String,
    pub creator_uuid: String,
    pub title: Option<String>,
    pub content_name: Option<String>,
    pub content_type: i16,
    pub keywords: String,
    pub description: String,
    pub display: bool,
    pub published: bool,
    pub published_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "contents"]
pub struct ContentInsert<'i> {
    pub uuid: &'i str,
    pub version: &'i str,
    pub creator_uuid: &'i str,
    pub title: Option<&'i str>,
    pub content_name: Option<&'i str>,
    pub content_type: i16,
    pub keywords: &'i str,
    pub description: &'i str,
    pub display: bool,
    pub published: bool,
    pub published_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "contents"]
pub struct ContentUpdate<'i> {
    pub version: Option<&'i str>,
    pub title: Option<&'i str>,
    pub content_name: Option<&'i str>,
    pub content_type: Option<i16>,
    pub keywords: Option<&'i str>,
    pub description: Option<&'i str>,
    pub display: Option<bool>,
    pub published: Option<bool>,
    pub published_at: Option<NaiveDateTime>,
}