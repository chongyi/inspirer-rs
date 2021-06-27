use inspirer_actix_ext::validator::Validate;

#[derive(Deserialize, Validate)]
pub struct FindContent {
    #[validate(range(min = 1))]
    pub id: u64
}

#[derive(Deserialize, Validate)]
pub struct CreateContent {
    pub draft: bool,
    #[validate(length(min = 1, max = 80))]
    pub title: String,
    #[validate(length(max = 140))]
    pub keywords: String,
    #[validate(length(max = 200))]
    pub description: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct DeleteOption {
    #[serde(default)]
    pub force: bool
}

#[derive(Deserialize, Validate)]
pub struct QueryContent {
    #[validate(range(min = 1))]
    pub page: Option<u64>,
    #[validate(range(min = 1, max = 50))]
    pub per_page: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, AsRefStr)]
#[serde(rename_all = "snake_case")]
pub enum ContentQuerySort {
    #[strum(serialize = "contents.id")]
    Id,
    #[strum(serialize = "contents.created_at")]
    CreatedAt,
    #[strum(serialize = "contents.updated_at")]
    UpdatedAt,
    #[strum(serialize = "contents.published_at")]
    PublishedAt,
}

