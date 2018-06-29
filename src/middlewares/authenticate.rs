use actix_web::{HttpRequest, HttpResponse, Result};
use actix_web::middleware::{Middleware, Started};
use actix_web::middleware::session::RequestSession;

use util::auth::PrivateClaims;
use util::message::ErrorMessage;

pub struct Authenticate;

impl<S> Middleware<S> for Authenticate {
    fn start(&self, req: &mut HttpRequest<S>) -> Result<Started> {
        if let Some(_) = req.session().get::<PrivateClaims>("claims")? {
            Ok(Started::Done)
        } else {
            Ok(Started::Response(HttpResponse::Unauthorized().json(ErrorMessage::<String> {
                code: 10014,
                msg: "Invalid authentication token.".to_owned(),
                body: None,
            })))
        }
    }
}