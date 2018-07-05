#[macro_use]
pub mod database;
pub mod auth;

use std::collections::HashMap;
use std::fmt;

use actix_web::{Error, ResponseError, HttpRequest, HttpResponse, HttpMessage, http::header::{HeaderValue, ACCEPT}};
use actix_web::http::{StatusCode};
use actix::MailboxError;
use failure::{Fail, Backtrace};
use futures::Canceled;

use super::message::ErrorMessage;
use self::database::DatabaseErrorKind;

#[macro_export]
macro_rules! error_handler {
    ($r:ident) => {
        {
            use actix_web::Error as IAErr;
            use util::error::{
                ApplicationError as IAppErr,
                JsonResponseError as JsonErr,
                TextResponseError as TextErr,
                RenderType as RType
            };
            let rt: RType = $r.into();
            let handler = move |err: IAppErr| -> IAErr {
                let info = match err {
                    IAppErr::SystemRuntimeError(info) => info,
                    IAppErr::AuthenticationError(info) => info,
                    IAppErr::DatabaseError(_, info) => info,
                };

                match rt {
                    RType::Json => JsonErr::new(info).into(),
                    RType::Text => TextErr::new(info).into(),
                }
            };

            handler
        }
    };
}

pub enum RenderType {
    Json,
    Text,
}

#[derive(Fail)]
pub enum ApplicationError {
    SystemRuntimeError(ErrorInformation),
    AuthenticationError(ErrorInformation),
    DatabaseError(DatabaseErrorKind, ErrorInformation),
}

impl From<Error> for ApplicationError {
    fn from(_: Error) -> Self {
        ApplicationError::SysLogicArgumentError()
    }
}

impl fmt::Debug for ApplicationError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error")
    }
}

impl fmt::Display for ApplicationError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error")
    }
}

impl ResponseError for ApplicationError {}

impl From<MailboxError> for ApplicationError {
    fn from(_: MailboxError) -> Self {
        ApplicationError::SysLogicArgumentError()
    }
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

impl ErrorInformation {
    pub fn new(code: u16, message: String, status: StatusCode, detail: Option<ErrorDetail>) -> Self {
        ErrorInformation { code, status, message, detail}
    }
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

pub fn error_handler<T, E>(render: T) -> Box<FnOnce(E) -> Error>
    where
        T: Into<RenderType>,
        E: Into<ApplicationError>,
{
    let rt = render.into();

    Box::new(move |error| {
        let err = error.into();
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

impl ApplicationError {
    pub const SYS_LOGIC_ARG_ERR: (u16, &'static str, StatusCode) = (10010, "Invalid argument.", StatusCode::INTERNAL_SERVER_ERROR);
    pub const SYS_INVALID_ARGUMENT_ERR: (u16, &'static str, StatusCode) = (10011, "Invalid argument.", StatusCode::BAD_REQUEST);

    #[allow(non_snake_case)]
    pub fn SysLogicArgumentError() -> Self {
        let (a, b, c) = Self::SYS_LOGIC_ARG_ERR;
        ApplicationError::SystemRuntimeError(ErrorInformation::new(
            a, b.into(), c, None
        ))
    }

    #[allow(non_snake_case)]
    pub fn SysInvalidArgumentError() -> Self {
        let (a, b, c) = Self::SYS_INVALID_ARGUMENT_ERR;
        ApplicationError::SystemRuntimeError(ErrorInformation::new(
            a, b.into(), c, None
        ))
    }
}