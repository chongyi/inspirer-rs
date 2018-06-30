use actix::*;
use actix_web::*;
use diesel;
use diesel::*;
use diesel::MysqlConnection;
use diesel::r2d2::{PooledConnection, ConnectionManager};
use chrono::NaiveDateTime;

use database::{DatabaseExecutor, Conn, last_insert_id};
use util::helper;
use util::message::{PaginatedListMessage, Pagination};
use schema::categories;

type PaginatedCategoryList = Result<PaginatedListMessage<CategoryDisplay>, Error>;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct CategoryBase {
    pub id: u32,
    pub name: String,
    pub display_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateCategory {
    pub name: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub sort: Option<i16>,
}

#[derive(Deserialize, Insertable, Debug)]
#[table_name = "categories"]
pub struct NewCategory {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub sort: Option<i16>,
}

impl From<CreateCategory> for NewCategory {
    fn from(origin: CreateCategory) -> Self {
        NewCategory {
            name: origin.name.clone(),
            display_name: origin.display_name.unwrap_or(origin.name.clone()),
            description: origin.description.unwrap_or("".to_owned()),
            sort: origin.sort
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct CategoryDisplay {
    pub id: u32,
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub sort: i16,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct Category;

impl Category {
    pub fn get_category_list(connection: &Conn, c: Pagination<GetCategoryList>) -> PaginatedCategoryList {
        use schema::categories::dsl::*;

        let paginator = paginator!(connection, c, CategoryDisplay, {
            let mut query = categories.into_boxed();
            if let Some(filter) = c.clone().filter {
                if let Some(v) = filter.name {
                    query = query.filter(name.like(format!("%{}%", v)));
                }
            }

            query.order((sort.desc(), created_at.desc(), id.desc()))
        });

        paginator()
    }

    pub fn create_category(connection: &Conn, category: NewCategory) -> Result<u64, Error> {
        use schema::categories::dsl::*;

        diesel::insert_into(categories)
            .values(category)
            .execute(connection)
            .map_err(error::ErrorInternalServerError)?;

        let generated_id: u64 = diesel::select(last_insert_id)
            .first(connection)
            .map_err(error::ErrorInternalServerError)?;

        Ok(generated_id)
    }

    pub fn delete_category(connection: &Conn, category_id: u32) -> Result<u32, Error> {
        use schema::categories::dsl::*;

        let count = diesel::delete(categories)
            .filter(id.eq(category_id))
            .execute(connection)
            .map_err(error::ErrorInternalServerError)?;

        Ok(count as u32)
    }
}

#[derive(Debug, Clone)]
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

impl Message for NewCategory {
    type Result = Result<u64, Error>;
}

impl Handler<NewCategory> for DatabaseExecutor {
    type Result = Result<u64, Error>;

    fn handle(&mut self, category: NewCategory, _: &mut Self::Context) -> Self::Result {
        Category::create_category(&self.connection()?, category)
    }
}

pub struct DeleteCategory (pub u32);

impl Message for DeleteCategory {
    type Result = Result<u32, Error>;
}

impl Handler<DeleteCategory> for DatabaseExecutor {
    type Result = Result<u32, Error>;

    fn handle(&mut self, finder: DeleteCategory, _: &mut Self::Context) -> Self::Result {
        Category::delete_category(&self.connection()?, finder.0)
    }
}