use inspirer_actix_ext::database::{CreateDAO, DAO, ReadDAO, UpdateDAO};
use inspirer_actix_ext::database::sqlx::{MySql, Executor, Done, Arguments, FromRow};
use crate::model::content::{NewContent, NewContentEntity, NewContentEntityWithContent, ContentEntityBasic, ContentBasic};
use sqlx::mysql::{MySqlArguments, MySqlRow};
use sqlx::Row;
use crate::dao::condition_str;
use inspirer_actix_ext::database::statement::IntoStatement;
use inspirer_actix_ext::database::statement::pagination::{Paginated, Paginate, IntoPaginated};
use inspirer_actix_ext::database::statement::sort::SortStatement;

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

pub enum LatestDraft {
    OfCreator(u64),
    OfContent(u64),
}

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

#[derive(Serialize, Deserialize, Debug, AsRefStr)]
#[serde(rename_all = "snake_case")]
pub enum ContentQuerySort {
    #[strum(serialize = "id")]
    Id,
    #[strum(serialize = "created_at")]
    CreatedAt,
    #[strum(serialize = "updated_at")]
    UpdatedAt,
    #[strum(serialize = "published_at")]
    PublishedAt
}

#[derive(Default)]
pub struct ContentQueryCondition {
    pub id: Option<u64>,
    pub creator_id: Option<u64>,
    pub is_display: Option<bool>,
    pub is_published: Option<bool>,
    pub paginate: Option<Paginate>,
    pub sort: SortStatement<ContentQuerySort>
}

#[async_trait]
impl ReadDAO<MySql, ContentBasic> for ContentQueryCondition {
    type Result = sqlx::Result<Paginated<ContentBasic>>;

    async fn read<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=MySql>
    {
        let mut arguments = MySqlArguments::default();
        let mut conditions = vec![];

        if let Some(id) = self.id {
            conditions.push("id = ?");
            arguments.add(id);
        }

        if let Some(creator_id) = self.creator_id {
            conditions.push("creator_id = ?");
            arguments.add(creator_id);
        }

        if let Some(is_display) = self.is_display {
            conditions.push("is_display = ?");
            arguments.add(is_display);
        }

        if let Some(is_published) = self.is_published {
            conditions.push("is_published = ?");
            arguments.add(is_published);
        }

        let mut sql = format!(
            "{} {} {}",
            include_str!("_sql_files/content/get_content_basic_list.sql"),
            condition_str(conditions),
            self.sort.full_statement()
        );

        if let Some(paginate) = self.paginate {
            sql.push_str(" limit ?,?");
            arguments.add(paginate.skip());
            arguments.add(paginate.take());
        }

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