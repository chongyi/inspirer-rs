use actix::{Message, Handler};
use diesel;
use diesel::*;
use chrono::NaiveDateTime;

use result::Result;
use database::{DatabaseExecutor, Conn, last_insert_id};
use message::{PaginatedListMessage, Pagination, UpdateByID};
use error::{Error, database::map_database_error};
use schema::push_messages;

#[derive(Deserialize, Insertable, Debug)]
#[table_name = "push_messages"]
pub struct NewPushMessage {
    pub content: String,
}

#[derive(Deserialize, AsChangeset, Debug)]
#[table_name = "push_messages"]
pub struct UpdatePushMessage {
    pub sort: Option<i16>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct PushMessageDisplay {
    pub id: u32,
    pub content: String,
    pub sort: i16,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

pub struct PushMessage;

impl PushMessage {

    pub fn create(connection: &Conn, data: NewPushMessage) -> Result<u32> {
        use schema::push_messages::dsl::*;

        diesel::insert_into(push_messages)
            .values(&data)
            .execute(connection)
            .map_err(map_database_error(Some("push_messages")))?;

        let generated_id: u64 = diesel::select(last_insert_id)
            .first(connection)
            .map_err(map_database_error(Some("push_messages")))?;

        Ok(generated_id as u32)
    }

    pub fn find_by_id(connection: &Conn, target: u32) -> Result<PushMessageDisplay> {
        use schema::push_messages::dsl::*;

        find_by_id!(connection => (
            push_messages # = target => PushMessageDisplay
        ))
    }

    pub fn update(connection: &Conn, target: u32, data: UpdatePushMessage) -> Result<Option<PushMessageDisplay>> {
        use schema::push_messages::dsl::*;

        let count = update_by_id!(connection => (
            push_messages # = target; <- &data
        ))?;

        if count > 0 {
            Ok(Self::find_by_id(connection, target).ok())
        } else {
            Ok(None)
        }
    }

    pub fn get_list(connection: &Conn, c: Pagination<GetPushMessages>) -> Result<PaginatedListMessage<PushMessageDisplay>> {
        use schema::push_messages::dsl::*;

        let paginator = paginator!(connection, c, PushMessageDisplay, {
            let mut query = push_messages.into_boxed();
            if let Some(filter) = c.clone().filter {
                if let Some(v) = filter.keywords {
                    query = query.filter(content.like(format!("%{}%", &v)));
                }
            }

            query.order((sort.desc(), created_at.desc()))
        });

        paginator()
    }

    pub fn delete(connection: &Conn, target: u32) -> Result<usize> {
        use schema::push_messages::dsl::*;

        let count = delete_by_id!(connection => (
            push_messages # = target
        ))?;

        Ok(count)
    }
}

#[derive(Clone, Debug)]
pub struct GetPushMessages {
    pub keywords: Option<String>,
}

impl Default for GetPushMessages {
    fn default() -> Self {
        GetPushMessages {
            keywords: None
        }
    }
}

impl Message for Pagination<GetPushMessages> {
    type Result = Result<PaginatedListMessage<PushMessageDisplay>>;
}

impl Handler<Pagination<GetPushMessages>> for DatabaseExecutor {
    type Result = <Pagination<GetPushMessages> as Message>::Result;

    fn handle(&mut self, msg: Pagination<GetPushMessages>, _: &mut Self::Context) -> Self::Result {
        PushMessage::get_list(&self.connection()?, msg)
    }
}

pub struct FindPushMessage(pub u32);

impl Message for FindPushMessage {
    type Result = Result<PushMessageDisplay>;
}

impl Handler<FindPushMessage> for DatabaseExecutor {
    type Result = <FindPushMessage as Message>::Result;

    fn handle(&mut self, msg: FindPushMessage, _: &mut Self::Context) -> Self::Result {
        PushMessage::find_by_id(&self.connection()?, msg.0)
    }
}