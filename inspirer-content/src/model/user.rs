pub use crate::entity::users::Model as UserModel;

#[derive(Debug, Clone, Default)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub nickname: String,
    pub avatar: String,
}