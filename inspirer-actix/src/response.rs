use actix_web::{Responder, HttpRequest, HttpResponse, Error};
use actix_web::http::header::ACCEPT;
use serde::Serialize;
use serde::export::PhantomData;
use actix_web::dev::HttpResponseBuilder;
use actix_web::error::{BlockingError};
use crate::error::CodedError;

#[derive(Serialize)]
pub struct ResponseMessage<'a, T>
    where T: Serialize
{
    pub code: i16,
    pub msg: &'a str,
    pub data: &'a T,
}

impl<'a, T> ResponseMessage<'a, T>
    where T: Serialize
{
    pub fn ok(data: &'a T) -> Self {
        ResponseMessage {
            code: 0,
            msg: "ok",
            data,
        }
    }

    pub fn error<E: AsRef<dyn CodedError>>(error: &'a E, data: &'a T) -> Self {
        ResponseMessage {
            code: error.as_ref().error_code(),
            msg: error.as_ref().error_message(),
            data,
        }
    }
}