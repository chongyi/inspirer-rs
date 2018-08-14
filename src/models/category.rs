use actix::*;
use actix_web::*;
use diesel;
use diesel::*;
use chrono::NaiveDateTime;

use database::{DatabaseExecutor, Conn, last_insert_id};
use message::{PaginatedListMessage, Pagination, UpdateByID};
use error::{Error, database::map_database_error};
use schema::categories;

type PaginatedCategoryList = Result<PaginatedListMessage<CategoryDisplay>, Error>;
type DisplayCategoryDetail = Result<Option<CategoryDisplay>, Error>;

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

    pub fn find_category_by_id(connection: &Conn, category_id: u32) -> Result<CategoryDisplay, Error> {
        use schema::categories::dsl::*;

        Ok(
            categories
                .filter(id.eq(category_id))
                .first::<CategoryDisplay>(connection)
                .map_err(map_database_error("categories"))?
        )
    }

    pub fn find_category_by_name(connection: &Conn, category_name: String) -> Result<CategoryDisplay, Error> {
        use schema::categories::dsl::*;

        Ok(
            categories
                .filter(name.eq(category_name))
                .first::<CategoryDisplay>(connection)
                .map_err(map_database_error("categories"))?
        )
    }

    pub fn create_category(connection: &Conn, category: NewCategory) -> Result<u64, Error> {
        use schema::categories::dsl::*;

        diesel::insert_into(categories)
            .values(category)
            .execute(connection)
            .map_err(map_database_error("categories"))?;

        let generated_id: u64 = diesel::select(last_insert_id)
            .first(connection)
            .map_err(map_database_error("categories"))?;

        Ok(generated_id)
    }

    pub fn delete_category(connection: &Conn, category_id: u32) -> Result<u32, Error> {
        use schema::categories::dsl::*;

        let count = diesel::delete(categories)
            .filter(id.eq(category_id))
            .execute(connection)
            .map_err(map_database_error("categories"))?;

        Ok(count as u32)
    }

    pub fn update_category(connection: &Conn, category_id: u32, update: UpdateCategory) -> DisplayCategoryDetail {
        use schema::categories::dsl::*;

        let count = diesel::update(categories)
            .set(&update)
            .filter(id.eq(category_id))
            .execute(connection)
            .map_err(map_database_error("categories"))?;

        if (count as u32) > 0 {
            Ok(Self::find_category_by_id(connection, category_id).ok())
        } else {
            Ok(None)
        }
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

#[derive(AsChangeset, Deserialize)]
#[table_name="categories"]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub sort: Option<i16>,
}

impl Message for UpdateByID<UpdateCategory> {
    type Result = DisplayCategoryDetail;
}

impl Handler<UpdateByID<UpdateCategory>> for DatabaseExecutor {
    type Result = DisplayCategoryDetail;

    fn handle(&mut self, update: UpdateByID<UpdateCategory>, _: &mut Self::Context) -> Self::Result {
        Category::update_category(&self.connection()?, update.id, update.update)
    }
}

pub enum  FindCategory {
    Id(u32),
    Name(String),
}

impl Message for FindCategory {
    type Result = DisplayCategoryDetail;
}

impl Handler<FindCategory> for DatabaseExecutor {
    type Result = DisplayCategoryDetail;

    fn handle(&mut self, find: FindCategory, _: &mut Self::Context) -> Self::Result {
        Ok(Some(match find {
            FindCategory::Id(id) => Category::find_category_by_id(&self.connection()?, id)?,
            FindCategory::Name(name) => Category::find_category_by_name(&self.connection()?, name)?,
        }))
    }
}
