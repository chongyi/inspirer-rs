pub mod database;
pub mod auth;
pub mod common;

use std::fmt;

use actix_web::{HttpResponse, Body, error::Error, HttpRequest, HttpMessage};
use actix_web::dev::HttpResponseBuilder;
use actix_web::error::{ResponseError, InternalError};
use failure::{Fail, Backtrace};
use mime::{Mime, APPLICATION_JSON, TEXT_HTML, TEXT_PLAIN};

use util::message::ErrorMessage;
pub use database::*;
use serde::Serialize;

pub type ErrorDesc = (u16, &'static str);

#[derive(Copy, Clone)]
pub enum RenderType {
    Json,
    Text,
}

impl<S> From<HttpRequest<S>> for RenderType {
    fn from(req: HttpRequest<S>) -> Self {
        match req.mime_type() {
            Ok(op) => match op {
                Some(mime) => {
                    if mime == APPLICATION_JSON {
                        RenderType::Json
                    } else {
                        RenderType::Text
                    }
                },
                None => RenderType::Text,
            },
            Err(_) => RenderType::Text,
        }
    }
}

pub struct ErrorContainer {
    cause: Box<RuntimeCause>,
}

pub fn error_container<T: RuntimeCause>(err: T) -> ErrorContainer {
    ErrorContainer {
        cause: Box::new(err),
    }
}

pub fn runtime_error_container<T, R>(render: R) -> Box<FnOnce(T) -> Error>
    where
        T: RuntimeCause,
        R: Into<RenderType>,
{
    let r = render.into();
    Box::new(move  |err: T| {
        RuntimeError::new(err, r).into()
    })
}

pub struct RuntimeError<T> {
    cause: T,
    render: RenderType,
    backtrace: Backtrace,
}

pub trait RuntimeCause: Fail + fmt::Display + fmt::Debug {
    fn render(&self, render: RenderType) -> HttpResponse;
}

impl<T> RuntimeError<T>
    where T: RuntimeCause
{
    pub fn new(cause: T, render: RenderType) -> Self {
        RuntimeError {
            cause,
            render,
            backtrace: Backtrace::new(),
        }
    }
}

impl<T> ResponseError for RuntimeError<T>
    where T: RuntimeCause
{
    fn error_response(&self) -> HttpResponse {
        self.cause.render(self.render)
    }
}

impl<T> Fail for RuntimeError<T>
    where
        T: Send + Sync + fmt::Debug + fmt::Display + 'static,
{
    fn backtrace(&self) -> Option<&Backtrace> {
        Some(&self.backtrace)
    }
}

impl<T> fmt::Debug for RuntimeError<T>
    where
        T: Send + Sync + fmt::Debug + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.cause, f)
    }
}

impl<T> fmt::Display for RuntimeError<T>
    where
        T: Send + Sync + fmt::Display + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.cause, f)
    }
}

#[derive(Fail, Debug, PartialEq)]
pub enum ApplicationLogicError {
    #[fail(display = "Invalid argument.")]
    InvalidArgument,
    #[fail(display = "System logic error.")]
    LogicError,
}

const APP_ERR_REQUEST_ERR: ErrorDesc = (10001, "Request error");
const APP_ERR_INVALID_ARGUMENT: ErrorDesc = (10004, "Invalid argument.");
const APP_ERR_LOGIC_ERR: ErrorDesc = (10001, "System logic error.");

impl RuntimeCause for ApplicationLogicError {
    fn render(&self, render: RenderType) -> HttpResponse {
        let (builder, (code, message)) = match *self {
            ApplicationLogicError::InvalidArgument => (HttpResponse::BadRequest(), APP_ERR_INVALID_ARGUMENT),
            ApplicationLogicError::LogicError => (HttpResponse::InternalServerError(), APP_ERR_LOGIC_ERR),
        };

        match render {
            RenderType::Json => builder.json(ErrorMessage::<String> {
                code,
                msg: "Runtime error.".to_string(),
                body: Some(message.to_string()),
            })
        }
    }
}