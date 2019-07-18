use crate::prelude::*;
use crate::schema::content_entities;

#[derive(Queryable, Debug, Clone, PartialEq, Serialize)]
pub struct ContentEntity {
    pub content_body: Option<String>,
    pub creator_uuid: Option<String>,
}

#[derive(Deserialize, Insertable)]
#[table_name = "content_entities"]
pub struct ContentEntityInsert<'i> {
    pub id: i64,
    pub version: &'i str,
    pub content_body: Option<&'i str>,
    pub creator_uuid: Option<&'i str>,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "content_entities"]
pub struct ContentEntityUpdate<'i> {
    pub version: &'i str,
    pub content_body: &'i str,
    pub creator_uuid: Option<&'i str>,
}