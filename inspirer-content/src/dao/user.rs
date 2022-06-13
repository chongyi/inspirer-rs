use sea_orm::{ConnectionTrait, EntityTrait, QueryFilter, Set, ColumnTrait};
use uuid::Uuid;

use crate::{entity::users, error::InspirerContentResult, model::user::NewUser};

#[async_trait::async_trait]
pub trait UserDao {
    async fn create_user(
        &self,
        id: Uuid,
        new_user: &NewUser,
        public_key: Vec<u8>,
        public_key_fingerprint: Vec<u8>,
    ) -> InspirerContentResult<Uuid>;

    async fn get_user_by_id(&self, id: Uuid) -> InspirerContentResult<Option<users::Model>>;
    async fn get_user_by_public_key(
        &self,
        fingerprint: Vec<u8>,
    ) -> InspirerContentResult<Option<users::Model>>;
    async fn get_user_by_username(&self, username: String) -> InspirerContentResult<Option<users::Model>>;
}

#[async_trait::async_trait]
impl<T: ConnectionTrait> UserDao for T {
    async fn create_user(
        &self,
        id: Uuid,
        new_user: &NewUser,
        public_key: Vec<u8>,
        public_key_fingerprint: Vec<u8>,
    ) -> InspirerContentResult<Uuid> {
        let model = users::ActiveModel {
            id: Set(id),
            username: Set(new_user.username.clone()),
            password: Set(new_user.password.clone()),
            nickname: Set(new_user.nickname.clone()),
            avatar: Set(new_user.avatar.clone()),
            public_key: Set(public_key),
            public_key_fingerprint: Set(public_key_fingerprint),
            ..Default::default()
        };

        users::Entity::insert(model).exec(self).await?;

        Ok(id)
    }

    async fn get_user_by_id(&self, id: Uuid) -> InspirerContentResult<Option<users::Model>> {
        users::Entity::find_by_id(id)
            .one(self)
            .await
            .map_err(Into::into)
    }

    async fn get_user_by_public_key(
        &self,
        fingerprint: Vec<u8>,
    ) -> InspirerContentResult<Option<users::Model>> {
        users::Entity::find()
            .filter(users::Column::PublicKeyFingerprint.eq(fingerprint))
            .one(self)
            .await
            .map_err(Into::into)
    }

    async fn get_user_by_username(&self, username: String) -> InspirerContentResult<Option<users::Model>> {
        users::Entity::find()
            .filter(users::Column::Nickname.eq(username))
            .one(self)
            .await
            .map_err(Into::into)
    }
}
