use std::collections::{HashMap, HashSet};

use actix::{Message, Handler};
use diesel;
use diesel::*;
use chrono::NaiveDateTime;

use result::Result;
use database::{DatabaseExecutor, Conn, last_insert_id};
use message::{PaginatedListMessage, Pagination, UpdateByID};
use error::{Error, database::map_database_error};

use schema::subjects;
use schema::subject_relates;

pub struct Subject;

#[derive(Deserialize, Insertable, Debug)]
#[table_name = "subjects"]
pub struct NewSubject {
    pub name: Option<String>,
    pub title: String,
    pub keywords: String,
    pub description: String,
    pub sort: i16,
}

#[derive(Deserialize, AsChangeset, Debug)]
#[table_name = "subjects"]
pub struct UpdateSubject {
    pub name: Option<String>,
    pub title: Option<String>,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub sort: Option<i16>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct SubjectDisplay {
    pub id: u32,
    pub name: Option<String>,
    pub title: String,
    pub keywords: String,
    pub description: String,
    pub sort: i16,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

pub struct SubjectRelate {
    pub content_id: u32,
    pub sort: Option<i16>,
}

pub struct CreateSubject {
    pub subject: NewSubject,
    pub relates: Option<Vec<SubjectRelate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct SubjectRelateBaseInfo {
    pub id: u32,
    pub name: Option<String>,
    pub title: String,
    pub category_id: Option<u32>,
    pub content_sort: i16,
    pub content_type: u16,
    pub display: bool,
    pub relate_sort: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct SubjectRelateInfo {
    pub id: u32,
    pub name: Option<String>,
    pub title: String,
    pub category_id: Option<u32>,
    pub description: String,
    pub keywords: String,
    pub content_sort: i16,
    pub content_type: u16,
    pub published_at: Option<NaiveDateTime>,
    pub modified_at: Option<NaiveDateTime>,
    pub display: bool,
    pub relate_sort: i16,
}

impl Subject {
    pub fn create(connection: &Conn, create: CreateSubject) -> Result<u32> {
        use schema::subjects::dsl::*;

        connection.transaction(move || {
            let rows: usize = diesel::insert_into(subjects)
                .values(&create.subject)
                .execute(connection)
                .map_err(map_database_error(Some("subjects")))?;

            if rows < 1 {
                return Err(Error::internal_server_error(Some("[unknown]"), None));
            }

            let generated_id: u64 = diesel::select(last_insert_id)
                .first(connection)
                .map_err(map_database_error(Some("subjects")))?;
            let generated_id = generated_id as u32;

            match create.relates {
                Some(relates) => {
                    use schema::subject_relates::dsl::*;

                    let mut create_relates = Vec::with_capacity(10);
                    for relate in relates {
                        create_relates.push(
                            (subject_id.eq(generated_id), content_id.eq(relate.content_id), sort.eq(relate.sort.unwrap_or(0)))
                        );
                    }

                    if create_relates.len() > 0 {
                        diesel::insert_into(subject_relates)
                            .values(&create_relates)
                            .execute(connection)
                            .map_err(map_database_error(Some("subject_relates")))?;
                    }
                }
                None => (),
            };

            Ok(generated_id)
        })
    }

    pub fn sync_relates(connection: &Conn, target: u32, sync: Vec<SubjectRelate>) -> Result<()> {
        use schema::subject_relates::dsl::*;

        let mut key_index = Vec::with_capacity(sync.len());

        for item in &sync {
            key_index.push(item.content_id);
        }

        connection.transaction(move || {
            diesel::delete(subject_relates)
                .filter(
                    subject_id.eq(target).and(content_id.ne_all(key_index))
                )
                .execute(connection)
                .map_err(map_database_error(Some("subject_relates")))?;

            let mut create_relates = Vec::with_capacity(sync.len());
            for relate in sync {
                create_relates.push(
                    (subject_id.eq(target), content_id.eq(relate.content_id), sort.eq(relate.sort.unwrap_or(0)))
                );
            }

            if create_relates.len() > 0 {
                diesel::replace_into(subject_relates)
                    .values(&create_relates)
                    .execute(connection)
                    .map_err(map_database_error(Some("subject_relates")))?;
            }

            Ok(())
        })
    }

    pub fn find_by_id(connection: &Conn, target: u32) -> Result<SubjectDisplay> {
        use schema::subjects::dsl::*;

        find_by_id!(connection => (
            subjects # = target => SubjectDisplay
        ))
    }

    pub fn find_by_name(connection: &Conn, target: String) -> Result<SubjectDisplay> {
        use schema::subjects::dsl::*;

        find_by_id!(connection => (
            subjects name = target => SubjectDisplay
        ))
    }

    pub fn update(connection: &Conn, target: u32, update: UpdateSubject) -> Result<Option<SubjectDisplay>> {
        use schema::subjects::dsl::*;

        let count = update_by_id!(connection => (
            subjects # = target; <- &update
        ))?;

        if count > 0 {
            Ok(Self::find_by_id(connection, target).ok())
        } else {
            Ok(None)
        }
    }

    pub fn delete(connection: &Conn, target: u32) -> Result<(usize, usize)> {
        use schema::subjects::dsl::*;

        connection.transaction(|| {
            let count = delete_by_id!(connection => (subjects # = target))?;
            if count > 0 {
                use schema::subject_relates::dsl::*;
                let relates_count = delete_by_id!(connection => (subject_relates subject_id = target))?;

                Ok((count, relates_count))
            } else {
                Ok((count, 0))
            }
        })
    }

    pub fn get_list(connection: &Conn) {}

    pub fn get_relate_list(connection: &Conn, paginated: Pagination<GetRelateList>) -> Result<PaginatedListMessage<SubjectRelateInfo>> {
        use schema::contents;
        use schema::subject_relates as sr;

        let target = paginated.filter.ok_or(Error::bad_request_error(Some("[param]"), None))?.target;
        let display = paginated.filter.ok_or(Error::bad_request_error(Some("[param]"), None))?.display;

        let paginator = paginator!(
            connection,
            (contents::id, contents::name, contents::title, contents::category_id, contents::description, contents::keywords, contents::sort, contents::content_type, contents::published_at, contents::modified_at, contents::display, sr::sort),
            paginated,
            SubjectRelateInfo,
            {

                sr::table
                    .inner_join(contents::table.on(
                        sr::content_id.eq(contents::id)
                            .and(contents::display.eq(display))
                    ))
                    .filter(sr::subject_id.eq(target))
                    .order_by((sr::sort.desc(), contents::sort.desc()))
            });

        paginator()
    }

    pub fn get_relate_base_info_list(connection: &Conn, target: u32, display: bool) -> Result<Vec<SubjectRelateBaseInfo>> {
        use schema::contents;
        use schema::subject_relates as sr;

        sr::table
            .inner_join(contents::table.on(
                sr::content_id.eq(contents::id)
                    .and(contents::display.eq(display))
            ))
            .select((contents::id, contents::name, contents::title, contents::category_id, contents::sort, contents::content_type, contents::display, sr::sort))
            .filter(sr::subject_id.eq(target))
            .order_by((sr::sort.desc(), contents::sort.desc()))
            .load::<SubjectRelateBaseInfo>(connection)
            .map_err(map_database_error(Some("subject_relates")))
    }
}

#[derive(Clone, Debug, Copy)]
pub struct GetRelateList {
    pub target: u32,
    pub display: bool
}