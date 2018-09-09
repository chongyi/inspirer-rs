use actix::{Message, Handler};
use diesel;
use diesel::*;
use diesel::dsl::exists;
use chrono::NaiveDateTime;

use result::Result;
use database::{DatabaseExecutor, Conn, last_insert_id};
use message::{PaginatedListMessage, Pagination, UpdateByID};
use error::{Error, database::map_database_error};
use schema::categories;
use schema::categories::dsl as column;
use regex::Regex;

type PaginatedCategoryList = Result<PaginatedListMessage<CategoryDisplay>>;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct CategoryBase {
    pub id: u32,
    pub name: String,
    pub display_name: String,
}

#[derive(Deserialize, Insertable, Debug)]
#[table_name = "categories"]
pub struct NewCategory {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub sort: Option<i16>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct CategoryDisplay {
    pub id: u32,
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct CategoryFullDisplay {
    pub id: u32,
    pub name: String,
    pub display_name: String,
    pub keywords: String,
    pub description: String,
    pub sort: i16,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<CategoryFullDisplay> for CategoryBase {
    fn from(origin: CategoryFullDisplay) -> Self {
        CategoryBase {
            id: origin.id,
            name: origin.name.clone(),
            display_name: origin.display_name.clone(),
        }
    }
}

pub struct Category;

impl Category {
    const DISPLAY_COLUMNS: (
        column::id, column::name, column::display_name,
        column::description, column::created_at, column::updated_at
    ) = (
        column::id, column::name, column::display_name,
        column::description, column::created_at, column::updated_at
    );

    pub fn get_list(connection: &Conn, c: Pagination<GetCategoryList>) -> PaginatedCategoryList {
        use schema::categories::dsl::*;

        let paginator = paginator!(connection, Self::DISPLAY_COLUMNS, c, CategoryDisplay, {
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

    pub fn find_by_id(connection: &Conn, category_id: u32) -> Result<CategoryFullDisplay> {
        use schema::categories::dsl::*;

        Ok(
            categories
                .filter(id.eq(category_id))
                .first::<CategoryFullDisplay>(connection)
                .map_err(map_database_error(Some("categories")))?
        )
    }

    pub fn find_by_name(connection: &Conn, category_name: String) -> Result<CategoryFullDisplay> {
        use schema::categories::dsl::*;

        Ok(
            categories
                .filter(name.eq(category_name))
                .first::<CategoryFullDisplay>(connection)
                .map_err(map_database_error(Some("categories")))?
        )
    }

    pub fn create(connection: &Conn, category: NewCategory) -> Result<u64> {
        use schema::categories::dsl::*;

        diesel::insert_into(categories)
            .values(category)
            .execute(connection)
            .map_err(map_database_error(Some("categories")))?;

        let generated_id: u64 = diesel::select(last_insert_id)
            .first(connection)
            .map_err(map_database_error(Some("categories")))?;

        Ok(generated_id)
    }

    pub fn delete(connection: &Conn, category_id: u32) -> Result<usize> {
        use schema::categories::dsl::*;

        let count = delete_by_id!(connection => (categories # = category_id))?;

        Ok(count)
    }

    pub fn update(connection: &Conn, category_id: u32, update: UpdateCategory) -> Result<Option<CategoryFullDisplay>> {
        use schema::categories::dsl::*;

        let count = update_by_id!(connection => (
            categories # = category_id; <- &update
        ))?;

        if count > 0 {
            Ok(Self::find_by_id(connection, category_id).ok())
        } else {
            Ok(None)
        }
    }

    pub fn exists(connection: &Conn, category: String) -> Result<bool> {
        use schema::categories::dsl::*;

        let regex = Regex::new(r"^\d+$").unwrap();

        if regex.is_match(&category) {
            let category_id = category.parse::<u32>().unwrap();
            select(exists(categories.filter(id.eq(category_id))))
                .get_result(connection).map_err(map_database_error(Some("categories")))
        } else {
            select(exists(categories.filter(name.eq(category))))
                .get_result(connection).map_err(map_database_error(Some("categories")))
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
        Category::get_list(&self.connection()?, condition)
    }
}

impl Message for NewCategory {
    type Result = Result<u64>;
}

impl Handler<NewCategory> for DatabaseExecutor {
    type Result = Result<u64>;

    fn handle(&mut self, category: NewCategory, _: &mut Self::Context) -> Self::Result {
        Category::create(&self.connection()?, category)
    }
}

pub struct DeleteCategory(pub u32);

impl Message for DeleteCategory {
    type Result = Result<usize>;
}

impl Handler<DeleteCategory> for DatabaseExecutor {
    type Result = Result<usize>;

    fn handle(&mut self, finder: DeleteCategory, _: &mut Self::Context) -> Self::Result {
        Category::delete(&self.connection()?, finder.0)
    }
}

#[derive(AsChangeset, Deserialize)]
#[table_name = "categories"]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub sort: Option<i16>,
}

impl Message for UpdateByID<UpdateCategory> {
    type Result = Result<Option<CategoryFullDisplay>>;
}

impl Handler<UpdateByID<UpdateCategory>> for DatabaseExecutor {
    type Result = Result<Option<CategoryFullDisplay>>;

    fn handle(&mut self, update: UpdateByID<UpdateCategory>, _: &mut Self::Context) -> Self::Result {
        Category::update(&self.connection()?, update.id, update.update)
    }
}

#[derive(Clone, Debug)]
pub enum FindCategory {
    Id(u32),
    Name(String),
}

impl From<String> for FindCategory {
    fn from(origin: String) -> Self {
        let result = origin.parse::<u32>();
        match result {
            Ok(v) => FindCategory::Id(v),
            Err(_) => FindCategory::Name(origin),
        }
    }
}

impl Message for FindCategory {
    type Result = Result<Option<CategoryFullDisplay>>;
}

impl Handler<FindCategory> for DatabaseExecutor {
    type Result = Result<Option<CategoryFullDisplay>>;

    fn handle(&mut self, find: FindCategory, _: &mut Self::Context) -> Self::Result {
        Ok(Some(match find {
            FindCategory::Id(id) => Category::find_by_id(&self.connection()?, id)?,
            FindCategory::Name(name) => Category::find_by_name(&self.connection()?, name)?,
        }))
    }
}
