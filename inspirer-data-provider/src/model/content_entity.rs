use crate::prelude::*;
use crate::schema::content_entities;

#[derive(Queryable, Debug, Clone, PartialEq)]
pub struct ContentEntity {
    pub content_body: Option<String>,
    pub creator_uuid: Option<String>,
}

#[derive(Deserialize, Insertable)]
#[table_name = "content_entities"]
pub struct ContentEntityInsert<'i> {
    pub id: i64,
    pub version: i32,
    pub content_body: Option<&'i str>,
    pub creator_uuid: Option<&'i str>,
}