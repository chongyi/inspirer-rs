//! 授权认证中间件
//!

use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use futures::future::{ok, Ready, Either};
use std::pin::Pin;
use std::future::Future;
use std::task::{Poll, Context};
use actix_web::http::header::AUTHORIZATION;
use actix_web::http::HeaderValue;
use failure::_core::marker::PhantomData;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::de::DeserializeOwned;
use actix_http::HttpMessage;
use actix_web::HttpResponse;
use crate::response::ResponseMessage;
use crate::error::{INVALID_OR_BAD_REQUEST, UNAUTHORIZED};

pub struct JwtTokenAuth<'a, K>
    where K: DeserializeOwned + 'static
{
    secret: &'a str,
    _phantom: PhantomData<K>,
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
                req.extensions_mut().insert(token);
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