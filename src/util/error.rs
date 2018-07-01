use std::fmt;

use actix_web::HttpResponse;
use actix_web::dev::HttpResponseBuilder;
use actix_web::error::{
    ResponseError,
    PayloadError,
    JsonPayloadError,
};
use diesel::result::Error as DatabaseError;
use diesel::result::DatabaseErrorKind;
use failure::{Fail, Backtrace};

use util::message::ErrorMessage;

pub struct RuntimeError<T> {
    cause: T,
    message: String,
    code: u16,
    backtrace: Backtrace,
}

impl<T> RuntimeError<T> {
    pub fn new(cause: T, message: String, code: u16) -> Self {
        RuntimeError {
            cause,
            message,
            code,
            backtrace: Backtrace::new(),
        }
    }

    pub fn json(&self, builder: &mut HttpResponseBuilder) -> HttpResponse {
        builder.json(ErrorMessage::<u8> {
            code: self.code,
            msg: self.message.clone(),
            body: None,
        })
    }

    pub fn custom<I>(&self, origin: (&mut HttpResponseBuilder, Option<u16>, Option<String>, Option<I>)) -> HttpResponse
        where I: ::serde::Serialize
    {
        let (builder, origin_code, message, body) = origin;
        let code = origin_code.unwrap_or(self.code);
        let msg = message.unwrap_or(self.message.clone());

        builder.json(ErrorMessage::<I> {
            code,
            msg,
            body,
        })
    }
}

impl ResponseError for RuntimeError<DatabaseError> {
    fn error_response(&self) -> HttpResponse {
        let (mut builder, code, message, body): (HttpResponseBuilder, Option<u16>, Option<String>, Option<u8>) =
            match self.cause {
                DatabaseError::NotFound => (HttpResponse::NotFound(), Some(51001), Some("No result".into()), None),
                DatabaseError::DatabaseError(
                    DatabaseErrorKind::UniqueViolation, _
                ) => (HttpResponse::Conflict(), Some(51005), Some("Data exists.".into()), None),
                _ => (HttpResponse::InternalServerError(), None, None, None),
            };

        self.custom((&mut builder, code, message, body))
    }
}

impl ResponseError for RuntimeError<PayloadError> {
    fn error_response(&self) -> HttpResponse {
        let mut builder: HttpResponseBuilder = match self.cause {
            _ => HttpResponse::InternalServerError(),
        };

        builder.json(ErrorMessage::<u8> {
            code: self.code,
            msg: self.message.clone(),
            body: None,
        })
    }
}

impl ResponseError for RuntimeError<JsonPayloadError> {
    fn error_response(&self) -> HttpResponse {
        match self.cause {
            JsonPayloadError::Overflow => HttpResponse::PayloadTooLarge().json(ErrorMessage::<u8> {
                code: 61011,
                msg: String::from("Too large."),
                body: None,
            }),
            _ => HttpResponse::BadRequest().json(ErrorMessage::<u8> {
                code: 61010,
                msg: String::from("Invalid parameters."),
                body: None,
            }),
        }
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