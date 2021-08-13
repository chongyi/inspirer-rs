use thiserror::Error;

#[derive(Error, Debug)]
pub enum InspirerContentError {
    #[error("Cannot create content from draft, save to entity failed.")]
    CannotCreateContentFromDraft
}