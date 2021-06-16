use inspirer_actix_ext::service::{IntoService, DependencyFactory};
use sqlx::MySqlPool;
use crate::dao::content::ContentQueryCondition;
use inspirer_actix_ext::database::{Get, DAO};
use crate::model::content::ContentBasic;
use inspirer_actix_ext::database::statement::pagination::Paginated;

#[derive(Service, FromRequest)]
pub struct ContentService {
    pool: MySqlPool
}

impl ContentService {
    pub async fn list(&self, query_condition: ContentQueryCondition) -> Paginated<ContentBasic> {
        Get::<ContentBasic>::by(query_condition)
            .run(&self.pool)
            .await
            .unwrap()
    }
}