#[derive(Serialize, Deserialize, Clone, Debug, sqlx::FromRow)]
pub struct UserBasic {
    pub id: u64,
    pub user_type: u16,
    pub username: String,
    pub nickname: String,
    #[serde()]
    pub password: String,
}