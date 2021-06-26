use inspirer_actix_ext::database::{CreateDAO, DAO, DeleteDAO, ReadDAO, UpdateDAO};
use inspirer_actix_ext::database::sqlx::{Arguments, Done, Executor, FromRow, MySql};
use inspirer_actix_ext::database::statement::IntoStatement;
use inspirer_actix_ext::database::statement::pagination::{IntoPaginated, Paginate, Paginated};
use inspirer_actix_ext::database::statement::sort::SortStatement;
use sqlx::mysql::{MySqlArguments, MySqlRow};
use sqlx::Row;

use crate::dao::condition_str;
use crate::model::content::{ContentBasic, ContentEntityBasic, ContentForClient, ContentForClientBasic, NewContent, NewContentEntity, NewContentEntityWithContent, NewContentMeta};
use crate::request::content::ContentQuerySort;
use chrono::{DateTime, Utc};

/// 创建内容
///
/// 用于初次创建内容时使用
#[async_trait]
impl<'s> CreateDAO<MySql> for NewContent<'s> {
    type Result = sqlx::Result<u64>;

    async fn create<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        sqlx::query(include_str!("_sql_files/content/create_content.sql"))
            .bind(self.creator_id)
            .bind(self.title)
            .bind(self.keywords)
            .bind(self.description)
            .bind(self.content_entity_id)
            .bind(self.is_display)
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
        sqlx::query(include_str!("_sql_files/content/create_new_content_entity.sql"))
            .bind(self.creator_id)
            .bind(self.is_draft)
            .bind(self.title)
            .bind(self.keywords)
            .bind(self.description)
            .bind(self.content)
            .execute(executor)
            .await
            .map(|result| result.last_insert_id())
    }
}

/// 在有内容的情况下，创建内容实体（创建即绑定内容）
///
/// 一般用于更新内容时使用
#[async_trait]
impl<'s> CreateDAO<MySql> for NewContentEntityWithContent<'s> {
    type Result = sqlx::Result<u64>;

    async fn create<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        sqlx::query(include_str!("_sql_files/content/create_content_entity.sql"))
            .bind(self.entity.creator_id)
            .bind(self.content_id)
            .bind(self.entity.is_draft)
            .bind(self.entity.title)
            .bind(self.entity.keywords)
            .bind(self.entity.description)
            .bind(self.entity.content)
            .execute(executor)
            .await
            .map(|result| result.last_insert_id())
    }
}

pub struct BindSource {
    pub content_id: u64,
    pub content_entity_id: u64,
}

pub struct BindContentEntityToContent(pub BindSource);

pub struct BindContentToContentEntity(pub BindSource);

/// 绑定内容实体至内容
///
/// 一般用于初次创建内容后的首次绑定
#[async_trait]
impl DAO<MySql> for BindContentEntityToContent {
    type Result = sqlx::Result<bool>;

    async fn run<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        sqlx::query(include_str!("_sql_files/content/bind_content_entity_to_content.sql"))
            .bind(self.0.content_id)
            .bind(self.0.content_entity_id)
            .execute(executor)
            .await
            .map(|result| result.rows_affected() > 0)
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
            .bind(self.0.content_entity_id)
            .bind(self.0.content_id)
            .execute(executor)
            .await
            .map(|result| result.rows_affected() > 0)
    }
}

/// 获取最新草稿
pub enum LatestDraft {
    /// 基于作者去查询最新草稿
    OfCreator(u64),
    /// 基于内容 ID 去查询
    OfContent(u64),
}

/// 获取最新草稿
#[async_trait]
impl ReadDAO<MySql, ContentEntityBasic> for LatestDraft {
    type Result = sqlx::Result<ContentEntityBasic>;

    async fn read<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        let (id, sql) = match self {
            LatestDraft::OfCreator(id) => (*id, include_str!("_sql_files/content/get_creator_latest_draft_basic.sql")),
            LatestDraft::OfContent(id) => (*id, include_str!("_sql_files/content/get_content_latest_draft_basic.sql")),
        };

        sqlx::query_as(sql)
            .bind(id)
            .fetch_one(executor)
            .await
    }
}

pub struct ContentFromEntity {
    pub content_entity_id: u64,
    pub without_draft: bool,
}

/// 更新内容信息
///
/// 读取内容实体，并将最新内容实体的信息同步至内容表
#[async_trait]
impl UpdateDAO<MySql> for ContentFromEntity {
    type Result = sqlx::Result<bool>;

    async fn update<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        sqlx::query(include_str!("_sql_files/content/update_content_from_entity.sql"))
            .bind(self.content_entity_id)
            .bind(if self.without_draft {
                0
            } else {
                1
            })
            .execute(executor)
            .await
            .map(|result| result.rows_affected() > 0)
    }
}

#[derive(Default)]
pub struct ContentQueryCondition {
    pub id: Option<u64>,
    pub creator_id: Option<u64>,
    pub is_display: Option<bool>,
    pub is_published: Option<bool>,
    pub is_deleted: Option<bool>,
    pub with_deleted: bool,
    pub paginate: Option<Paginate>,
    pub sort: SortStatement<ContentQuerySort>,
}

impl ContentQueryCondition {
    pub fn build(&self, sql: &str) -> (String, MySqlArguments) {
        let mut arguments = MySqlArguments::default();
        let mut conditions = vec![];

        if let Some(id) = self.id {
            conditions.push("contents.id = ?");
            arguments.add(id);
        }

        if let Some(creator_id) = self.creator_id {
            conditions.push("contents.creator_id = ?");
            arguments.add(creator_id);
        }

        if let Some(is_display) = self.is_display {
            conditions.push("contents.is_display = ?");
            arguments.add(is_display);
        }

        if let Some(is_published) = self.is_published {
            conditions.push("contents.is_published = ?");
            arguments.add(is_published);
        }

        if !self.with_deleted {
            conditions.push("contents.is_deleted = ?");
            if let Some(is_deleted) = self.is_deleted {
                arguments.add(is_deleted);
            } else {
                arguments.add(false);
            }
        }

        let mut sql = format!(
            "{} {} {}",
            sql,
            condition_str(conditions),
            self.sort.full_statement()
        );

        if let Some(paginate) = self.paginate {
            sql.push_str(" limit ?,?");
            arguments.add(paginate.skip());
            arguments.add(paginate.take());
        }

        (sql, arguments)
    }
}

#[async_trait]
impl ReadDAO<MySql, ContentBasic> for ContentQueryCondition {
    type Result = sqlx::Result<Paginated<ContentBasic>>;

    async fn read<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        let (sql, arguments) = self.build(include_str!("_sql_files/content/get_content_basic_list.sql"));
        let list = sqlx::query_with(sql.as_str(), arguments)
            .try_map(|row: MySqlRow| {
                let content = ContentBasic::from_row(&row)?;
                let total: i64 = row.try_get("total")?;
                Ok((content, total))
            })
            .fetch_all(executor)
            .await?;

        Ok(list.raw_into(self.paginate))
    }
}

#[async_trait]
impl ReadDAO<MySql, ContentForClientBasic> for ContentQueryCondition {
    type Result = sqlx::Result<Paginated<ContentForClientBasic>>;

    async fn read<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        let (sql, arguments) = self.build(include_str!("_sql_files/content/get_content_for_client_basic_list.sql"));
        let list = sqlx::query_with(sql.as_str(), arguments)
            .try_map(|row: MySqlRow| {
                let content = ContentForClientBasic::from_row(&row)?;
                let total: i64 = row.try_get("total")?;
                Ok((content, total))
            })
            .fetch_all(executor)
            .await?;

        Ok(list.raw_into(self.paginate))
    }
}

pub enum Key {
    Id(u64),
}

#[async_trait]
impl ReadDAO<MySql, ContentBasic> for Key {
    type Result = sqlx::Result<ContentBasic>;

    async fn read<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        let mut argument = MySqlArguments::default();
        let sql = match self {
            Key::Id(id) => {
                argument.add(*id);
                concat!(include_str!("_sql_files/content/find_contennt_basic.sql"), " where id = ? limit 1")
            }
        };

        sqlx::query_as_with(sql, argument)
            .fetch_one(executor)
            .await
    }
}

/// 删除内容
#[async_trait]
impl DeleteDAO<MySql> for Key {
    type Result = sqlx::Result<u64>;

    async fn delete<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        let query = match self {
            Key::Id(id) => sqlx::query(include_str!("_sql_files/content/delete_content.sql"))
                .bind(id)
        };

        query.execute(executor)
            .await
            .map(|result| result.rows_affected())
    }

    async fn force_delete<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        let query = match self {
            Key::Id(id) => sqlx::query(include_str!("_sql_files/content/force_delete_content.sql"))
                .bind(id)
        };

        query.execute(executor)
            .await
            .map(|result| result.rows_affected())
    }
}

pub struct ContentEntityByKey(pub Key);

#[async_trait]
impl DeleteDAO<MySql> for ContentEntityByKey {
    type Result = sqlx::Result<u64>;

    async fn delete<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        let query = match self.0 {
            Key::Id(id) => sqlx::query(include_str!("_sql_files/content/delete_content_entities_by_content_id.sql"))
                .bind(id)
        };

        query.execute(executor)
            .await
            .map(|result| result.rows_affected())
    }
}

#[async_trait]
impl<'s> UpdateDAO<MySql> for NewContentMeta<'s> {
    type Result = sqlx::Result<bool>;

    async fn update<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        sqlx::query(include_str!("_sql_files/content/update_content_meta.sql"))
            .bind(self.content_entity_id)
            .bind(self.title)
            .bind(self.keywords)
            .bind(self.description)
            .bind(self.id)
            .execute(executor)
            .await
            .map(|result| result.rows_affected() > 0)
    }
}

#[async_trait]
impl ReadDAO<MySql, ContentForClient> for Key {
    type Result = sqlx::Result<ContentForClient>;

    async fn read<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        let query = match self {
            Key::Id(id) => sqlx::query_as(include_str!("_sql_files/content/get_content_for_client.sql"))
                .bind(id)
        };

        query.fetch_one(executor)
            .await
    }
}