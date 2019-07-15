use actix_web::{Responder, HttpRequest, HttpResponse, Error};
use actix_web::http::header::ACCEPT;
use serde::Serialize;
use serde::export::PhantomData;
use actix_web::dev::HttpResponseBuilder;
use actix_web::error::{BlockingError};
use crate::error::CodedError;

#[derive(Serialize)]
pub enum InspirerResp<T>
    where T: Serialize
{
    Template { data: T, template: String },
    Json(T),
}

impl<T> InspirerResp<T>
    where T: Serialize
{
    pub fn with_template(data: T, template: String) -> Self {
        InspirerResp::Template { data, template }
    }

    pub fn json(data: T) -> Self {
        InspirerResp::Json(data)
    }
}

impl<T> Responder for InspirerResp<T>
    where T: Serialize
{
    type Error = actix_web::Error;
    type Future = Result<HttpResponse, Self::Error>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        match self {
            InspirerResp::Template { data, template } => {
                match req.headers().get(ACCEPT) {
                    Some(value) => {
                        let v = value.to_str().unwrap_or("");
                        if v.contains("application/json") {
                            return Ok(HttpResponse::Ok().json(&ResponseMessage::ok(&data)));
                        }
                    }
                    _ => ()
                }
            }
            InspirerResp::Json(data) => return Ok(HttpResponse::Ok().json(&ResponseMessage::ok(&data)))
        }

        Ok(HttpResponse::Ok().body("empty"))
    }
}

#[derive(Serialize)]
pub struct ResponseMessage<'a, T>
    where T: Serialize
{
    code: i16,
    msg: &'a str,
    data: &'a T,
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

    pub fn error<E: AsRef<CodedError>>(error: &'a E, data: &'a T) -> Self {
        ResponseMessage {
            code: error.as_ref().error_code(),
            msg: error.as_ref().error_message(),
            data,
        }
    }
}