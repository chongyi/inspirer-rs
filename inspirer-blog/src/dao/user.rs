use inspirer_actix_ext::database::ReadDAO;
use inspirer_actix_ext::database::sqlx::{MySql, Executor};
use crate::model::user::UserBasic;
use inspirer_actix_ext::database::sqlx::query::QueryAs;
use inspirer_actix_ext::database::sqlx::mysql::MySqlArguments;

pub enum Key<'a> {
    Id(u64),
    Username(&'a str),
}

fn get_user_basic_handler<'a>(key: &'a Key<'_>) -> QueryAs<'a, MySql, UserBasic, MySqlArguments> {
    match key {
        Key::Id(id) => sqlx::query_as(include_str!("_sql_files/user/get_user_basic_by_id.sql"))
            .bind(*id),
        Key::Username(username) => sqlx::query_as(include_str!("_sql_files/user/get_user_basic_by_username.sql"))
            .bind(username)
    }
}

#[async_trait]
impl<'s> ReadDAO<MySql, Option<UserBasic>> for Key<'s> {
    type Result = sqlx::Result<Option<UserBasic>>;

    async fn read<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        get_user_basic_handler(self)
            .fetch_optional(executor)
            .await
    }
}

#[async_trait]
impl<'s> ReadDAO<MySql, UserBasic> for Key<'s> {
    type Result = sqlx::Result<UserBasic>;

    async fn read<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        get_user_basic_handler(self)
            .fetch_one(executor)
            .await
    }
}