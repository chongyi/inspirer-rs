use sqlx::{Done, Executor, MySql};

use inspirer_query_ext::dao::{CreateDAO, DAO, ReadDAO, UpdateDAO, DeleteDAO};

use crate::model::{ContentEntityMeta, NewContent, NewContentEntity, UpdateContentEntity, BindContentToContentEntity, GetLatestContentEntity, ContentEntityFull, DeleteContent, DeleteContentEntityByContentId};

/// 创建内容
///
/// 用于初次创建内容时使用
#[async_trait]
impl CreateDAO<MySql> for NewContent {
    type Result = sqlx::Result<u64>;

    async fn create<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        sqlx::query(include_str!("_sql_files/content/create_content.sql"))
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
        sqlx::query(include_str!("_sql_files/content/create_content_entity.sql"))
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

/// 绑定内容至内容实体
///
/// 一般用于提交草稿或更新内容
#[async_trait]
impl DAO<MySql> for BindContentToContentEntity {
    type Result = sqlx::Result<bool>;

    async fn run<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        sqlx::query(include_str!("_sql_files/content/bind_content_to_content_entity.sql"))
            .bind(self.content_entity_id)
            .bind(self.content_id)
            .execute(executor)
            .await
            .map(|result| result.rows_affected() > 0)
    }
}

/// 获取最新的内容实体
///
/// 参数传入 `content id` 即可
#[async_trait]
impl ReadDAO<MySql, ContentEntityMeta> for GetLatestContentEntity {
    type Result = sqlx::Result<Option<ContentEntityMeta>>;

    async fn read<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        sqlx::query_as(include_str!("_sql_files/content/get_latest_entity_meta_by_content_id.sql"))
            .bind(self.content_id)
            .bind(self.is_draft)
            .fetch_optional(executor)
            .await
    }
}

/// 更新内容实体
///
/// 基本上仅用于草稿的处理
#[async_trait]
impl<'s> UpdateDAO<MySql> for UpdateContentEntity<'s> {
    type Result = sqlx::Result<bool>;

    async fn update<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        sqlx::query(include_str!("_sql_files/content/update_entity.sql"))
            .bind(self.content_id)
            .bind(self.is_draft)
            .bind(self.entity.title)
            .bind(self.entity.keywords)
            .bind(self.entity.description)
            .bind(self.entity.content)
            .bind(self.id)
            .execute(executor)
            .await
            .map(|result| result.rows_affected() > 0)
    }
}

#[async_trait]
impl ReadDAO<MySql, ContentEntityFull> for GetLatestContentEntity {
    type Result = sqlx::Result<Option<ContentEntityFull>>;

    async fn read<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        sqlx::query_as(include_str!("_sql_files/content/get_latest_entity_by_content_id.sql"))
            .bind(self.content_id)
            .bind(self.is_draft)
            .fetch_optional(executor)
            .await
    }
}

#[async_trait]
impl DeleteDAO<MySql> for DeleteContent {
    type Result = sqlx::Result<u64>;

    async fn delete<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        sqlx::query(include_str!("_sql_files/content/delete_content.sql"))
            .bind(self.0)
            .execute(executor)
            .await
            .map(|result| result.rows_affected())
    }

    async fn force_delete<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        sqlx::query(include_str!("_sql_files/content/force_delete_content.sql"))
            .bind(self.0)
            .execute(executor)
            .await
            .map(|result| result.rows_affected())
    }
}

#[async_trait]
impl DeleteDAO<MySql> for DeleteContentEntityByContentId {
    type Result = sqlx::Result<u64>;

    async fn delete<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        sqlx::query(include_str!("_sql_files/content/delete_content_entity_by_content_id.sql"))
            .bind(self.0)
            .execute(executor)
            .await
            .map(|result| result.rows_affected())
    }
}