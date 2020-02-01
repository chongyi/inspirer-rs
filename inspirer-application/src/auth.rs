#[derive(Serialize, Deserialize)]
pub struct Credential {
    pub uuid: String,
    pub exp: usize,
}