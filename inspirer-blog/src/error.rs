use std::collections::HashMap;
use std::fmt;

use actix_web::{HttpResponse, ResponseError};
use actix_web::body::Body;
use actix_web::dev::Service;
use actix_web::http::StatusCode;
use actix_web::web::BytesMut;
use serde::Serialize;
use validator::{ValidationErrors, ValidationErrorsKind};

const UNKNOWN_ERROR_CODE: i32 = 1;
const REQUEST_PARAMS_ERROR: i32 = 2;
const UNAUTHORIZED_TOKEN: i32 = 8;
const DATABASE_OTHER_ERROR: i32 = 1001;
const DATABASE_RESOURCE_NOT_FOUND: i32 = 1002;
const DATABASE_CONFLICT: i32 = 1003;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Serialize, Debug)]
pub struct ErrorResponse<T> {
    #[serde(skip_serializing)]
    pub http_status: StatusCode,
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> ErrorResponse<T>
    where T: Serialize
{
    pub fn to_response(&self) -> HttpResponse {
        HttpResponse::build(self.http_status.clone())
            .json(self)
    }
}

pub trait AsErrorResponse: fmt::Display + Sized {
    type Data: Serialize;

    fn http_status(&self) -> StatusCode;
    fn code(&self) -> i32;
    fn msg(&self) -> String {
        format!("{}", self)
    }
    fn data(&self) -> Option<&Self::Data> {
        None
    }
    fn as_error_response(&self) -> ErrorResponse<&Self::Data> {
        ErrorResponse {
            http_status: self.http_status(),
            code: self.code(),
            msg: self.msg(),
            data: self.data(),
        }
    }
    fn to_response(&self) -> HttpResponse {
        self.as_error_response().to_response()
    }
}

impl AsErrorResponse for sqlx::Error {
    type Data = ();

    fn http_status(&self) -> StatusCode {
        match self.code() {
            DATABASE_CONFLICT => StatusCode::CONFLICT,
            DATABASE_RESOURCE_NOT_FOUND => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn code(&self) -> i32 {
        match self {
            sqlx::Error::RowNotFound => DATABASE_RESOURCE_NOT_FOUND,
            sqlx::Error::Database(db_err) => db_err.code().map(|code| if code == "23000" {
                DATABASE_CONFLICT
            } else {
                DATABASE_OTHER_ERROR
            })
                .unwrap_or(DATABASE_OTHER_ERROR),
            _ => DATABASE_OTHER_ERROR
        }
    }

    fn msg(&self) -> String {
        match self.code() {
            DATABASE_CONFLICT => "Commit data failed",
            DATABASE_RESOURCE_NOT_FOUND => "Resource not found",
            _ => "System runtime error",
        }.into()
    }

    fn data(&self) -> Option<&Self::Data> {
        None
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Anyhow(anyhow::Error),
    #[error("{0}")]
    ValidateError(inspirer_actix_ext::validator::Error),
    #[error("{0}")]
    RuntimeError(RuntimeError),
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::Anyhow(err)
    }
}

impl From<inspirer_actix_ext::validator::Error> for Error {
    fn from(err: inspirer_actix_ext::validator::Error) -> Self {
        Error::ValidateError(err)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Anyhow(raw_err) => {
                None
                    .or_else(|| raw_err.downcast_ref::<RuntimeError>()
                        .map(|err| err.http_status()))
                    .or_else(|| raw_err.downcast_ref::<sqlx::Error>()
                        .map(|err| err.http_status()))
                    .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
            Error::ValidateError(err) => err.http_status(),
            Error::RuntimeError(err) => err.http_status(),
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            Error::Anyhow(raw_err) => {
                None
                    .or_else(|| raw_err.downcast_ref::<RuntimeError>()
                        .map(|err| err.to_response()))
                    .or_else(|| raw_err.downcast_ref::<sqlx::Error>()
                        .map(|err| err.to_response()))
                    .unwrap_or(RuntimeError::UnknownError.to_response())
            }
            Error::ValidateError(err) => err.to_response(),
            Error::RuntimeError(err) => err.to_response(),
        }
    }
}

#[derive(thiserror::Error, Debug, Copy, Clone)]
#[repr(i32)]
pub enum RuntimeError {
    #[error("Unknown server error.")]
    UnknownError = UNKNOWN_ERROR_CODE,
    #[error("Unauthorized, invalid token.")]
    InvalidToken = UNAUTHORIZED_TOKEN,
    #[error("User is not exists or password error.")]
    UserIsNotExists = 101001,
    #[error("User is not exists or password error.")]
    PasswordVerifiedError = 101002,
}

impl AsErrorResponse for RuntimeError {
    type Data = ();

    fn http_status(&self) -> StatusCode {
        match self {
            RuntimeError::UnknownError => StatusCode::INTERNAL_SERVER_ERROR,
            RuntimeError::InvalidToken => StatusCode::UNAUTHORIZED,
            RuntimeError::UserIsNotExists | RuntimeError::PasswordVerifiedError => StatusCode::BAD_REQUEST,
        }
    }

    fn code(&self) -> i32 {
        *self as i32
    }

    fn data(&self) -> Option<&Self::Data> {
        None
    }
}

impl ResponseError for RuntimeError {
    fn status_code(&self) -> StatusCode {
        self.http_status()
    }

    fn error_response(&self) -> HttpResponse {
        self.as_error_response().to_response()
    }
}

impl<'a> AsErrorResponse for inspirer_actix_ext::validator::Error {
    type Data = HashMap<&'static str, ValidationErrorsKind>;

    fn http_status(&self) -> StatusCode {
        self.status_code()
    }

    fn code(&self) -> i32 {
        REQUEST_PARAMS_ERROR
    }

    fn msg(&self) -> String {
        String::from("Request parameters error.")
    }

    fn data(&self) -> Option<&Self::Data> {
        Some(self.errors())
    }
}