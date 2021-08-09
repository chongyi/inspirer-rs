use sqlx::{MySql, Executor, Done};
use crate::model::{NewContent, NewContentEntity};
use inspirer_query_ext::dao::{CreateDAO, DAO};

/// 创建内容
///
/// 用于初次创建内容时使用
#[async_trait]
impl CreateDAO<MySql> for NewContent {
    type Result = sqlx::Result<u64>;

    async fn create<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        sqlx::query(include_str!("_sql_files/create_content.sql"))
            .bind(self.author_id)
            .execute(executor)
            .await
            .map(|result| result.last_insert_id())
    }
}

/// 创建内容实体
///
/// 用于初次创建内容时使用或创建草稿时使用
#[async_trait]
impl<'s> CreateDAO<MySql> for NewContentEntity<'s> {
    type Result = sqlx::Result<u64>;

    async fn create<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        sqlx::query(include_str!("_sql_files/create_content_entity.sql"))
            .bind(self.author_id)
            .bind(self.content_id)
            .bind(self.is_draft)
            .bind(self.entity.title)
            .bind(self.entity.keywords)
            .bind(self.entity.description)
            .bind(self.entity.content)
            .execute(executor)
            .await
            .map(|result| result.last_insert_id())
    }
}

pub struct BindContentToContentEntity {
    pub content_id: u64,
    pub content_entity_id: u64,
}

/// 绑定内容至内容实体
///
/// 一般用于提交草稿或更新内容
#[async_trait]
impl DAO<MySql> for BindContentToContentEntity {
    type Result = sqlx::Result<bool>;

    async fn run<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        sqlx::query(include_str!("_sql_files/bind_content_to_content_entity.sql"))
            .bind(self.content_entity_id)
            .bind(self.content_id)
            .execute(executor)
            .await
            .map(|result| result.rows_affected() > 0)
    }
}