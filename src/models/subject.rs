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

pub struct SubjectRelate {
    pub content_id: u32,
    pub sort: Option<i16>,
}

pub struct CreateSubject {
    pub subject: NewSubject,
    pub relates: Option<Vec<SubjectRelate>>,
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
                return Err(Error);
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
}