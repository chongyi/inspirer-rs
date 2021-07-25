pub use inspirer_content_common::dao::user::Key;

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}
