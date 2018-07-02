pub mod database;

use std::fmt;

use actix_web::{HttpResponse, Body, error::Error, HttpRequest, HttpMessage};
use actix_web::dev::HttpResponseBuilder;
use actix_web::error::{ResponseError, InternalError};
use failure::{Fail, Backtrace};
use mime::{Mime, APPLICATION_JSON, TEXT_HTML, TEXT_PLAIN};

use util::message::ErrorMessage;
pub use database::*;
use serde::Serialize;

#[derive(Copy, Clone)]
pub enum RenderType {
    Json,
    Text,
}

impl From<HttpRequest> for RenderType {
    fn from(req: HttpRequest<()>) -> Self {
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

pub fn runtime_error_container<T, R>(render: R) -> Box<Fn(T) -> Error>
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