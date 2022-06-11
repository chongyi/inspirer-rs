use uuid::Uuid;

use crate::{
    dao::user::UserDao,
    entity::users,
    error::InspirerContentResult,
    manager::Manager,
    model::user::NewUser,
    util::{
        signature::{generate_pkcs8_keypair, private_key_to_pem},
        uuid::generate_v1_uuid,
    },
};

#[async_trait::async_trait]
pub trait UserService {
    async fn create_user_simple(&self, new_user: NewUser) -> InspirerContentResult<(Uuid, String)>;
    async fn get_user_by_username(&self, username: String) -> InspirerContentResult<Option<users::Model>>;
}

#[async_trait::async_trait]
impl UserService for Manager {
    async fn create_user_simple(&self, new_user: NewUser) -> InspirerContentResult<(Uuid, String)> {
        // 生成 ID
        let id = generate_v1_uuid()?;

        // 生成私钥公钥
        let key_pair = generate_pkcs8_keypair()?;
        let public_key_fingerprint = key_pair.public_key_fingerprint();

        let public_key = key_pair.public_key;
        let private_key = key_pair.private_key;

        self.database
            .create_user(id, &new_user, public_key, public_key_fingerprint)
            .await?;

        Ok((id, private_key_to_pem(&private_key)?))
    }

    async fn get_user_by_username(
        &self,
        username: String,
    ) -> InspirerContentResult<Option<users::Model>> {
        self.database.get_user_by_username(username).await
    }
}
