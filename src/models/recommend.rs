use actix::{Message, Handler};
use diesel;
use diesel::*;
use diesel::dsl::exists;
use chrono::NaiveDateTime;

use result::Result;
use database::{DatabaseExecutor, Conn, last_insert_id};
use message::{PaginatedListMessage, Pagination, UpdateByID};
use error::{Error, database::map_database_error};
use schema::recommend_contents;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable)]
pub struct RecommendContentDisplay {
    pub id: u32,
    pub content_id: Option<u32>,
    pub source: String,
    pub title: String,
    pub summary: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Insertable, Debug)]
#[table_name = "recommend_contents"]
pub struct NewRecommendContent {
    pub content_id: u32,
    pub source: String,
    pub title: String,
    pub summary: String,
}

pub struct RecommendContent;

impl RecommendContent {
    pub fn push(connection: &Conn, origin_id: u32, summary: Option<String>) -> Result<u32> {
        use super::content::Content;

        let result = Content::find_by_id(connection, origin_id, None)?;
        let id = result.id.to_string();
        let name = result.name.as_ref();
        let name = name.unwrap_or(&id);
        let source = if (&result).as_page {
            format!("/{}", name)
        } else {
            format!("/article/{}", name)
        };


        let summary = if let Some(summary) = summary {
            summary
        } else {
            result.description.clone()
        };

        let creator = NewRecommendContent {
            content_id: origin_id,
            source,
            title: result.title.clone(),
            summary
        };

        Self::create(connection, creator)
    }

    pub fn create(connection: &Conn, data: NewRecommendContent) -> Result<u32> {
        use schema::recommend_contents::dsl::*;

        diesel::insert_into(recommend_contents)
            .values(&data)
            .execute(connection)
            .map_err(map_database_error(Some("recommend_contents")))?;

        let generated_id: u64 = diesel::select(last_insert_id)
            .first(connection)
            .map_err(map_database_error(Some("recommend_contents")))?;

        Ok(generated_id as u32)
    }

    pub fn get_recommend_contents(connection: &Conn, count: Option<u32>) -> Result<Vec<RecommendContentDisplay>> {
        use schema::recommend_contents::dsl::*;

        recommend_contents
            .order((created_at.desc(), id.desc()))
            .limit(count.map(From::from).unwrap_or(3))
            .load::<RecommendContentDisplay>(connection)
            .map_err(map_database_error(Some("recommend_contents")))
    }
}

#[derive(Default)]
pub struct GetRecommendContents(pub Option<u32>);

impl Message for GetRecommendContents {
    type Result = Result<Vec<RecommendContentDisplay>>;
}

impl Handler<GetRecommendContents> for DatabaseExecutor {
    type Result = <GetRecommendContents as Message>::Result;

    fn handle(&mut self, msg: GetRecommendContents, _: &mut Self::Context) -> <Self as Handler<GetRecommendContents>>::Result {
        RecommendContent::get_recommend_contents(&self.connection()?, msg.0)
    }
}