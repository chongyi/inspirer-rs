use inspirer_actix_ext::service::{IntoService, DependencyFactory};
use sqlx::MySqlPool;
use crate::dao::content::{ContentQueryCondition, Key};
use inspirer_actix_ext::database::{Get, DAO};
use crate::model::content::ContentBasic;
use inspirer_actix_ext::database::statement::pagination::Paginated;
use anyhow::Result;

#[derive(Service, FromRequest)]
pub struct ContentService {
    pool: MySqlPool
}

impl ContentService {
    pub async fn find(&self, key: Key) -> Result<ContentBasic> {
        Get::<ContentBasic>::by(key)
            .run(&self.pool)
            .await
            .map_err(Into::into)
    }

    pub async fn list(&self, query_condition: ContentQueryCondition) -> Result<Paginated<ContentBasic>> {
        Get::<ContentBasic>::by(query_condition)
            .run(&self.pool)
            .await
            .map_err(Into::into)
    }
}