use actix_web::HttpResponse;
use biscuit::errors::Error as BiscuitError;

use super::{RuntimeCause, RenderType, ErrorDesc};
use util::message::ErrorMessage;

const AUTH_VALIDATE_ERR: ErrorDesc = (10021, "Authentication invalidate.");

#[derive(Fail, Debug, PartialEq)]
pub enum AuthenticateError {
    #[fail(display = "Authentication invalidate.")]
    ValidateError
}

impl RuntimeCause for AuthenticateError {
    fn render(&self, render: RenderType) -> HttpResponse {
        let (builder, (code, message)) = match *self {
            AuthenticateError::ValidateError => (HttpResponse::Forbidden(), AUTH_VALIDATE_ERR),
        };

        match render {
            RenderType::Json => builder.json(ErrorMessage::<String> {
                code,
                msg: "Forbidden.".to_string(),
                body: Some(message.to_string())
            }),
            RenderType::Text => builder.body(message.to_string()),
        }
    }
}

impl RuntimeCause for BiscuitError {
    fn render(&self, render: RenderType) -> HttpResponse {
        let (builder, (code, message)) = match *self {
            _ => (HttpResponse::Forbidden(), AUTH_VALIDATE_ERR),
        };

        match render {
            RenderType::Json => builder.json(ErrorMessage::<String> {
                code,
                msg: "Forbidden.".to_string(),
                body: Some(message.to_string())
            }),
            RenderType::Text => builder.body(message.to_string()),
        }
    }
}