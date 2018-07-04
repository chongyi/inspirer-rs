pub mod database;

use std::collections::HashMap;
use std::fmt;

use actix_web::{Error, ResponseError, HttpRequest, HttpResponse, HttpMessage, http::header::{HeaderValue, ACCEPT}};
use actix_web::http::{StatusCode};
use failure::{Fail, Backtrace};

use self::database::DatabaseErrorKind;
use super::message::ErrorMessage;

pub enum RenderType {
    Json,
    Text,
}

pub enum ApplicationError {
    SystemRuntimeError(ErrorInformation),
    AuthenticationError(ErrorInformation),
    DatabaseError(DatabaseErrorKind, ErrorInformation),
}

#[derive(Fail, Debug, Serialize, Clone)]
pub enum ErrorDetail {
    String(String),
    Array(HashMap<String, ErrorDetail>),
}

impl fmt::Display for ErrorDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Fail, Debug)]
pub struct ErrorInformation {
    code: u16,
    status: StatusCode,
    message: String,
    detail: Option<ErrorDetail>,
}

impl Default for ErrorInformation {
    fn default() -> Self {
        ErrorInformation {
            code: 65535,
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: String::from("Unknown server error."),
            detail: None,
        }
    }
}

impl fmt::Display for ErrorInformation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "*Error*({}) {} \n{:?}", self.code, self.message, self.detail)
    }
}

pub fn error_handler<T>(render: T) -> Box<FnOnce(ApplicationError) -> Error>
    where T: Into<RenderType>
{
    let rt = render.into();

    Box::new(move |err: ApplicationError| {
        let info = match err {
            ApplicationError::SystemRuntimeError(info) => info,
            ApplicationError::AuthenticationError(info) => info,
            ApplicationError::DatabaseError(_, info) => info,
        };

        match rt {
            RenderType::Json => JsonResponseError::new(info).into(),
            RenderType::Text => TextResponseError::new(info).into(),
        }
    })
}

pub struct JsonResponseError {
    info: ErrorInformation,
    backtrace: Backtrace,
}

impl JsonResponseError {
    pub fn new(info: ErrorInformation) -> Self {
        JsonResponseError {
            info,
            backtrace: Backtrace::new(),
        }
    }
}

impl ResponseError for JsonResponseError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.info.status).json(ErrorMessage::new(self.info.code, self.info.message.clone(), Some(self.info.detail.clone())))
    }
}

impl Fail for JsonResponseError
{
    fn backtrace(&self) -> Option<&Backtrace> {
        Some(&self.backtrace)
    }
}

impl fmt::Debug for JsonResponseError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.info, f)
    }
}

impl fmt::Display for JsonResponseError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.info, f)
    }
}

pub struct TextResponseError {
    info: ErrorInformation,
    backtrace: Backtrace,
}

impl TextResponseError {
    pub fn new(info: ErrorInformation) -> Self {
        TextResponseError {
            info,
            backtrace: Backtrace::new(),
        }
    }
}

impl ResponseError for TextResponseError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.info.status).body(format!("<strong>Error</strong>({}) {}<br/>\n{:?}", self.info.code, self.info.message, self.info.detail))
    }
}

impl Fail for TextResponseError
{
    fn backtrace(&self) -> Option<&Backtrace> {
        Some(&self.backtrace)
    }
}

impl fmt::Debug for TextResponseError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.info, f)
    }
}

impl fmt::Display for TextResponseError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.info, f)
    }
}

impl<S> From<HttpRequest<S>> for RenderType {
    fn from(req: HttpRequest<S>) -> Self {
        let default = HeaderValue::from_static("text/html");
        let accept = req.headers()
            .get(ACCEPT).unwrap_or(&default)
            .to_str().unwrap_or("text/html");

        if accept.contains("json") {
            RenderType::Json
        } else {
            RenderType::Text
        }
    }
}