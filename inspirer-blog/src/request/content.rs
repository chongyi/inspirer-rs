use inspirer_actix_ext::validator::Validate;
use crate::constant::{QUERY_DEFAULT_PAGE, QUERY_DEFAULT_PER_PAGE};
use inspirer_actix_ext::database::statement::sort::SortStatement;

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
#[serde(default)]
pub struct ClientQueryContent {
    #[validate(range(min = 1))]
    pub page: u64,
    #[validate(range(min = 1, max = 50))]
    pub per_page: u64,
}

impl Default for ClientQueryContent {
    fn default() -> Self {
        ClientQueryContent {
            page: QUERY_DEFAULT_PAGE,
            per_page: QUERY_DEFAULT_PER_PAGE
        }
    }
}

#[derive(Deserialize, Validate)]
#[serde(default)]
pub struct AdminQueryContent {
    #[validate(range(min = 1))]
    pub page: u64,
    #[validate(range(min = 1, max = 50))]
    pub per_page: u64,
    pub is_deleted: bool,
    pub is_published: Option<bool>,
    pub is_display: Option<bool>,
    pub sorts: Option<SortStatement<ContentQuerySort>>
}

impl Default for AdminQueryContent {
    fn default() -> Self {
        AdminQueryContent {
            page: QUERY_DEFAULT_PAGE,
            per_page: QUERY_DEFAULT_PER_PAGE,
            is_deleted: false,
            is_published: None,
            is_display: None,
            sorts: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, AsRefStr, Clone)]
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

