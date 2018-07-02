use dotenv::Error as DotenvError;
use actix_web::HttpResponse;
use actix_web::dev::HttpResponseBuilder;
use actix_web::error::*;

use super::{RuntimeCause, RenderType, ApplicationLogicError, ErrorMessage, APP_ERR_REQUEST_ERR};

impl RuntimeCause for DotenvError {
    fn render(&self, render: RenderType) -> HttpResponse {
        let error = ApplicationLogicError::LogicError;
        error.render(render)
    }
}

impl RuntimeCause for ParseError {
    fn render(&self, render: RenderType) -> HttpResponse {
        let response = self.error_response();
        let mut builder = response.into_builder();
        let (code, message) = APP_ERR_REQUEST_ERR;

        match render {
            RenderType::Json => builder.json(ErrorMessage::<String> {
                code,
                msg: message.to_string(),
                body: None,
            }),
            RenderType::Text => builder.body(""),
        }
    }
}