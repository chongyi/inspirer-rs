use axum::{response::IntoResponse, http::StatusCode, Json};
use inspirer_content::error::Error as InspirerContentError;

use crate::response::error_message_from_err;

pub type InspirerResult<T, E = InspirerError> = Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum InspirerError {
    #[error(transparent)]
    InspirerContentError(#[from] inspirer_content::error::Error),
    #[error("请重新登录")]
    Unauthorized,
    #[error("创建 Token 失败")]
    CreateTokenError,
    #[error("Token 解析失败")]
    ParseTokenError,
}

impl IntoResponse for InspirerError {
    fn into_response(self) -> axum::response::Response {
        let msg = error_message_from_err(&self);
        let status = match self {
            InspirerError::InspirerContentError(InspirerContentError::ContentNotFound) => StatusCode::NOT_FOUND,
            InspirerError::InspirerContentError(InspirerContentError::DatabaseWriteConflict) => StatusCode::CONFLICT,
            InspirerError::InspirerContentError(InspirerContentError::UserNotFoundOrPasswordError) | InspirerError::ParseTokenError => StatusCode::FORBIDDEN,
            InspirerError::Unauthorized => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, Json(msg)).into_response()
    }
}