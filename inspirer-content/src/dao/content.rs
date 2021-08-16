use sqlx::{Executor, MySql, FromRow, Error, Arguments};

use inspirer_query_ext::dao::{CreateDAO, DAO, ReadDAO, UpdateDAO, DeleteDAO};

use crate::model::{ContentEntityMeta, NewContent, NewContentEntity, UpdateContentEntity, BindContentToContentEntity, GetLatestContentEntity, ContentEntityFull, DeleteContent, DeleteContentEntityByContentId, ContentWithEntity, ContentId, ContentBasic, ContentEntity, ContentWithEntitySummary, AdvanceContentQuery, ContentEntitySummary};
use sqlx::mysql::{MySqlRow, MySqlArguments};
use inspirer_query_ext::model::{PaginateWrapper, PaginationWrapper, RawPaginationWrapper};
use inspirer_query_ext::statement::IntoStatement;

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

impl<'r> FromRow<'r, MySqlRow> for ContentWithEntity {
    fn from_row(row: &'r MySqlRow) -> Result<Self, Error> {
        Ok(ContentWithEntity {
            content: ContentBasic::from_row(row)?,
            entity: ContentEntity::from_row(row)?,
        })
    }
}

#[async_trait]
impl ReadDAO<MySql, ContentWithEntity> for ContentId {
    type Result = sqlx::Result<Option<ContentWithEntity>>;

    async fn read<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        sqlx::query_as(include_str!("_sql_files/content/get_content_with_entity.sql"))
            .bind(self.0)
            .fetch_optional(executor)
            .await
    }
}

impl<'r> FromRow<'r, MySqlRow> for ContentWithEntitySummary {
    fn from_row(row: &'r MySqlRow) -> Result<Self, Error> {
        Ok(ContentWithEntitySummary {
            content: ContentBasic::from_row(row)?,
            entity: ContentEntitySummary::from_row(row)?,
        })
    }
}

#[async_trait]
impl ReadDAO<MySql, ContentWithEntitySummary> for PaginateWrapper<AdvanceContentQuery> {
    type Result = sqlx::Result<PaginationWrapper<Vec<ContentWithEntitySummary>>>;

    async fn read<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {

        let mut conditions = vec![];
        // create query parameters
        let mut arguments = MySqlArguments::default();

        // sql generate
        if let Some(ids) = self.id.as_ref() {
            ids.iter()
                .for_each(|inner| {
                    (&mut arguments).add(inner);
                });

            conditions.push(format!(
                "id in ({})",
                std::iter::repeat("?").take(ids.len())
                    .collect::<Vec<&str>>()
                    .join(",")
            ));
        }

        if let Some(is_deleted) = self.is_deleted {
            conditions.push("is_deleted = ?".into());
            arguments.add(is_deleted);
        }

        if let Some(is_published) = self.is_published {
            conditions.push("is_published = ?".into());
            arguments.add(is_published);
        }

        if let Some(is_display) = self.is_display {
            conditions.push("is_display = ?".into());
            arguments.add(is_display);
        }

        arguments.add(self.skip());
        arguments.add(self.take());

        let sql = format!(
            "{} {} {} limit ?, ?",
            include_str!("_sql_files/content/get_content_list_with_entity_summary.sql"),
            (!conditions.is_empty())
                .then(|| {
                    format!("where {}", conditions.join(" and "))
                })
                .unwrap_or(String::new()),
            self.sort.full_statement(),
        );

        let result: Vec<RawPaginationWrapper<ContentWithEntitySummary>> = sqlx::query_as_with(sql.as_str(), arguments)
            .fetch_all(executor)
            .await?;

        Ok(self.paginate.wrapped_pagination(result))
    }
}