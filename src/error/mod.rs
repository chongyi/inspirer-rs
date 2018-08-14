use std::borrow::BorrowMut;
use actix_web::{HttpRequest, HttpResponse, HttpMessage};
use actix_web::error::{Error as ActixError, ResponseError};
use mime;

pub mod database;

pub fn error_handler<T: AsRef<HttpRequest>>(req: T) -> impl FnOnce(Error) -> ActixError {
    let json = if let Ok(Some(mime)) = req.as_ref().mime_type() {
        mime.subtype() == mime::JSON || mime.suffix() == Some(mime::JSON)
    } else {
        false
    };

    move |err: Error| {
        if json {
            let json = JsonError;
            json.into();
        } else {
            let html = HtmlError;
            html.into();
        }
    }
}

pub struct Error;

pub struct JsonError;

pub struct HtmlError;

impl ResponseError for JsonError {

}

impl ResponseError for HtmlError {

}