use uuid::Uuid;

use crate::{
    dao::user::UserDao,
    entity::users,
    error::{Error, InspirerContentResult},
    manager::Manager,
    model::user::NewUser,
    util::{
        hash::{password_hash, verify_password},
        signature::{generate_pkcs8_keypair, private_key_to_pem},
        uuid::generate_v1_uuid,
    },
};

#[async_trait::async_trait]
pub trait UserService {
    async fn create_user_simple(&self, new_user: NewUser) -> InspirerContentResult<(Uuid, String)>;
    async fn get_user_by_id(&self, id: Uuid) -> InspirerContentResult<Option<users::Model>>;
    async fn get_user_by_username(
        &self,
        username: String,
    ) -> InspirerContentResult<Option<users::Model>>;
    async fn attempt(
        &self,
        username: String,
        password: String,
    ) -> InspirerContentResult<users::Model>;
}

#[async_trait::async_trait]
impl UserService for Manager {
    async fn create_user_simple(
        &self,
        mut new_user: NewUser,
    ) -> InspirerContentResult<(Uuid, String)> {
        // 生成 ID
        let id = generate_v1_uuid()?;

        // 生成私钥公钥
        let key_pair = generate_pkcs8_keypair()?;
        let public_key_fingerprint = key_pair.public_key_fingerprint();

        let public_key = key_pair.public_key;
        let private_key = key_pair.private_key;

        // 对 Password 进行 hash
        new_user.password = password_hash(new_user.password.as_str())?;

        self.database
            .create_user(id, &new_user, public_key, public_key_fingerprint)
            .await?;

        Ok((id, private_key_to_pem(&private_key)?))
    }

    async fn get_user_by_id(
        &self,
        id: Uuid,
    ) -> InspirerContentResult<Option<users::Model>> {
        self.database.get_user_by_id(id).await
    }

    async fn get_user_by_username(
        &self,
        username: String,
    ) -> InspirerContentResult<Option<users::Model>> {
        self.database.get_user_by_username(username).await
    }

    async fn attempt(
        &self,
        username: String,
        password: String,
    ) -> InspirerContentResult<users::Model> {
        let user = self
            .get_user_by_username(username)
            .await?
            .ok_or(Error::UserNotFoundOrPasswordError)?;

        if verify_password(&password, user.password.as_str())? {
            Ok(user)
        } else {
            Err(Error::UserNotFoundOrPasswordError)
        }
    }
}
