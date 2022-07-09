pub use inspirer_content::model::content::{
    NewContent as CreateContent,
    UpdateContent
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ForceDelete {
    pub force_delete: bool
}