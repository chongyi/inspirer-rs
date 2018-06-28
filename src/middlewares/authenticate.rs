use actix_web::{App, HttpRequest, HttpResponse, Result};
use actix_web::middleware::{Middleware, Started, Response};
use actix_web::middleware::session::RequestSession;
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use biscuit::*;
use biscuit::jws::*;
use biscuit::jwa::*;

pub struct Authenticate;

//impl<S> Middleware<S> for Authenticate {
//    fn start(&self, req: &mut HttpRequest<S>) -> Result<Started> {
//        let auth = Authorization::<Bearer>::parse(&req)?;
//
//    }
//}