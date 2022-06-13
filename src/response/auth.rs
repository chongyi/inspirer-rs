use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AccessToken {
    pub access_token: String,
}

#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub id: String,
    pub username: String,
    pub nickname: String,
    pub avatar: String,
}