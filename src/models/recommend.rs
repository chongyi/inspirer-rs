use actix::{Message, Handler};
use diesel;
use diesel::*;
use diesel::dsl::exists;
use chrono::NaiveDateTime;

use result::Result;
use database::{DatabaseExecutor, Conn, last_insert_id};
use message::{PaginatedListMessage, Pagination, UpdateByID};
use error::{Error, database::map_database_error};

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

pub struct RecommendContent;

impl RecommendContent {
    pub fn push(connection: &Conn, origin_id: u32, description: Option<String>) {

    }

    pub fn create(connection: &Conn) {

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