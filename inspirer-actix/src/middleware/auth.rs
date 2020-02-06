//! 授权认证中间件
//!

use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_http::{Error, HttpMessage};
use actix_http::error::PayloadError;
use actix_service::{Service, Transform};
use actix_web::{FromRequest, HttpRequest, HttpResponse};
use actix_web::dev::{Payload, ServiceRequest, ServiceResponse};
use actix_web::http::header::AUTHORIZATION;
use actix_web::http::HeaderValue;
use actix_web::web::Bytes;
use failure::_core::marker::PhantomData;
use futures::future::{Either, ok, Ready};
use futures::Stream;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::de::DeserializeOwned;

use crate::error::{INVALID_OR_BAD_REQUEST, UNAUTHORIZED};
use crate::response::ResponseMessage;

pub struct JwtToken<'a, K>
{
    secret: &'a str,
    _phantom: PhantomData<K>,
}

impl<'a, K> JwtToken<'a, K>
    where K: DeserializeOwned + 'static
{
    pub fn new(secret: &'a str) -> Self {
        JwtToken { secret, _phantom: PhantomData }
    }
}

impl<'a, S, B, K> Transform<S> for JwtToken<'a, K>
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=actix_web::Error>,
        S::Future: 'static,
        B: 'static,
        K: DeserializeOwned + 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = JwtTokenMiddleware<'a, S, K>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtTokenMiddleware { service, secret: self.secret, _phantom: PhantomData })
    }
}

pub struct JwtTokenMiddleware<'a, S, K> {
    secret: &'a str,
    service: S,
    _phantom: PhantomData<K>,
}

impl<'a, S, B, K> Service for JwtTokenMiddleware<'a, S, K>
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=actix_web::Error>,
        S::Future: 'static,
        B: 'static,
        K: DeserializeOwned + 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = S::Future;

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

                        Some(TokenGuard::<K>::from_token_str(token_str, self.secret.as_ref()))
                    } else {
                        None
                    }
                })
        };

        req.extensions_mut().insert(token_decode.unwrap_or_default());
        self.service.call(req)
    }
}

pub struct TokenInner<K>
    where K: DeserializeOwned + 'static
{
    data: Option<K>,
    origin: String,
}

pub struct TokenGuard<K: DeserializeOwned + 'static> (Rc<TokenInner<K>>);

impl<K> Default for TokenGuard<K>
    where K: DeserializeOwned + 'static
{
    fn default() -> Self {
        TokenGuard::<K> (Rc::new(TokenInner::<K> {
            data: None,
            origin: String::default(),
        }))
    }
}

impl<K: DeserializeOwned + 'static> TokenGuard<K> {
    pub(crate) fn from_token_str(token_str: &str, secret: &[u8]) -> Self {
        TokenGuard(Rc::new(TokenInner {
            data: decode::<K>(
                token_str,
                &DecodingKey::from_secret(secret),
                &Validation::default(),
            ).ok().map(|v| v.claims),
            origin: token_str.into(),
        }))
    }

    pub fn has_token(&self) -> bool {
        self.0.as_ref().data.is_some()
    }

    pub fn get_token(&self) -> Option<&K> {
        self.0.as_ref().data.as_ref()
    }

    pub fn get_origin_token(&self) -> &str {
        self.0.as_ref().origin.as_str()
    }
}

impl<K: DeserializeOwned + 'static> FromRequest for TokenGuard<K> {
    type Error = actix_web::Error;
    type Future = Ready<Result<TokenGuard<K>, actix_web::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ok(req.extensions().get::<TokenGuard<K>>().map(|v| {
            TokenGuard(Rc::clone(&v.0))
        }).unwrap())
    }
}