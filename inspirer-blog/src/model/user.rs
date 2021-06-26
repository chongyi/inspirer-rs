use std::ops::Deref;

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::FromRow)]
pub struct UserBasic {
    pub id: u64,
    pub user_type: u16,
    pub username: String,
    pub nickname: String,
    #[serde(skip_serializing)]
    pub password: String,
}

/// 用于用户 Json Web Token 的模型
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct UserTokenPayload {
    pub id: u64,
    pub user_type: u16,
}


impl From<&UserBasic> for UserTokenPayload {
    fn from(user_basic: &UserBasic) -> Self {
        UserTokenPayload {
            id: user_basic.id,
            user_type: user_basic.user_type,
        }
    }
}