use inspirer_actix_ext::service::{IntoService, DependencyFactory};
use inspirer_actix_ext::database::sqlx::MySqlPool;
use inspirer_content_common::dao::user::Key;
use inspirer_actix_ext::database::{Get, DAO};
use inspirer_content_common::model::user::UserBasic;
use anyhow::Result;

#[derive(FromRequest, Service)]
pub struct UserService {
    pool: MySqlPool
}

impl UserService {
    pub fn new(pool: MySqlPool) -> Self {
        UserService {
            pool
        }
    }

    pub async fn get_user_basic_optional(&self, key: Key<'_>) -> Result<Option<UserBasic>> {
        Ok(
            Get::<Option<UserBasic>>::by(key)
                .run(&self.pool)
                .await?
        )
    }

    pub async fn get_user_basic(&self, key: Key<'_>) -> Result<UserBasic> {
        Ok(
            Get::<UserBasic>::by(key)
                .run(&self.pool)
                .await?
        )
    }
}