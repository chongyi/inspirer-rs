//! 授权认证中间件
//!

use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse, Payload};
use futures::future::{ok, Ready, Either};
use std::pin::Pin;
use std::future::Future;
use std::task::{Poll, Context};
use actix_web::http::header::AUTHORIZATION;
use actix_web::http::HeaderValue;
use failure::_core::marker::PhantomData;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::de::DeserializeOwned;
use actix_http::{HttpMessage, Error};
use actix_web::{HttpResponse, FromRequest, HttpRequest};
use crate::response::ResponseMessage;
use crate::error::{INVALID_OR_BAD_REQUEST, UNAUTHORIZED};
use std::rc::Rc;
use actix_web::web::Bytes;
use futures::Stream;
use actix_http::error::PayloadError;

pub struct JwtTokenAuth<'a, K>
    where K: DeserializeOwned + 'static
{
    secret: &'a str,
    _phantom: PhantomData<K>,
}

impl<'a, K> JwtTokenAuth<'a, K>
    where K: DeserializeOwned + 'static
{
    pub fn new(secret: &'a str) -> Self {
        JwtTokenAuth { secret, _phantom: PhantomData }
    }
}

impl<'a, S, B, K> Transform<S> for JwtTokenAuth<'a, K>
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=actix_web::Error>,
        S::Future: 'static,
        B: 'static,
        K: DeserializeOwned + 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = JwtTokenAuthMiddleware<'a, S, K>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtTokenAuthMiddleware { service, secret: self.secret, _phantom: PhantomData })
    }
}

pub struct JwtTokenAuthMiddleware<'a, S, K>
    where K: DeserializeOwned + 'static
{
    secret: &'a str,
    service: S,
    _phantom: PhantomData<K>,
}

impl<'a, S, B, K> Service for JwtTokenAuthMiddleware<'a, S, K>
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=actix_web::Error>,
        S::Future: 'static,
        B: 'static,
        K: DeserializeOwned + 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let token_decode = {
            req.headers()
                .get(AUTHORIZATION)
                .and_then(|token| {
                    HeaderValue::to_str(token).ok()
                })
                .and_then(|token_raw| {
                    let split_result: Vec<&str> = token_raw.split(' ').collect();
                    if split_result.len() == 2 && &split_result[0] == &"Bearer" {
                        let token_str = split_result[1];

                        // Decode
                        decode::<K>(
                            &token_str,
                            &DecodingKey::from_secret(self.secret.as_ref()),
                            &Validation::default(),
                        ).ok()
                    } else {
                        None
                    }
                })
        };

        match token_decode {
            Some(token) => {
                let claims = token.claims;
                req.extensions_mut().insert(TokenGuard (Rc::new(claims)));
                Either::Left(self.service.call(req))
            }
            None => Either::Right(ok(req.into_response(
                HttpResponse::Unauthorized()
                    .json(ResponseMessage::<Option<bool>> {
                        code: UNAUTHORIZED.0,
                        msg: UNAUTHORIZED.1,
                        data: &None,
                    })
                    .into_body()
            )))
        }
    }
}

pub struct TokenGuard<K: DeserializeOwned + 'static> (pub Rc<K>);

impl<K: DeserializeOwned + 'static> FromRequest for TokenGuard<K> {
    type Error = actix_web::Error;
    type Future = Ready<Result<TokenGuard<K>, actix_web::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ok(req.extensions().get::<TokenGuard<K>>().map(|v| {
            TokenGuard (Rc::clone(&v.0))
        }).unwrap())
    }
}