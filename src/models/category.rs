use actix::*;
use actix_web::*;
use diesel::*;
use diesel::MysqlConnection;
use diesel::r2d2::{PooledConnection, ConnectionManager};
use chrono::NaiveDateTime;

use database::{DatabaseExecutor, Conn};
use util::helper;
use util::message::{PaginatedListMessage, Pagination};

type PaginatedCategoryList = Result<PaginatedListMessage<CategoryDisplay>, Error>;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct CategoryBase {
    pub id: u32,
    pub name: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct CategoryDisplay {
    pub id: u32,
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub sort: u16,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct Category;

impl Category {
    pub fn get_category_list(connection: &Conn, c: Pagination<GetCategoryList>) -> PaginatedCategoryList {
        use schema::categories::dsl::*;

        let paginator = paginator!(connection, c, CategoryDisplay, {
            let mut query = categories.into_boxed();
            if let Some(filter) = c.clone() {
                if let Some(v) = filter.name {
                    query = query.filter(name.like(format!("%{}%", v)));
                }
            }

            query.order((created_at.desc(), id.desc()))
        });

        paginator()
    }
}

pub struct GetCategoryList {
    pub name: Option<String>,
}

impl Message for Pagination<GetCategoryList> {
    type Result = PaginatedCategoryList;
}

impl Handler<Pagination<GetCategoryList>> for DatabaseExecutor {
    type Result = PaginatedCategoryList;

    fn handle(&mut self, condition: Pagination<GetCategoryList>, _: &mut Self::Context) -> Self::Result {
        Category::get_category_list(&self.connection()?, condition)
    }
}