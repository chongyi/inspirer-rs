use axum::{response::IntoResponse, http::StatusCode, Json};
use inspirer_content::error::Error as InspirerContentError;

use crate::response::error_message_from_err;

pub type InspirerResult<T, E = InspirerError> = Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum InspirerError {
    #[error(transparent)]
    InspirerContentError(#[from] inspirer_content::error::Error),
}

impl IntoResponse for InspirerError {
    fn into_response(self) -> axum::response::Response {
        let msg = error_message_from_err(&self);
        let status = match self {
            InspirerError::InspirerContentError(err) => match err {
                InspirerContentError::ContentNotFound => StatusCode::NOT_FOUND,
                InspirerContentError::DatabaseWriteConflict => StatusCode::CONFLICT,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        };

        (status, Json(msg)).into_response()
    }
}