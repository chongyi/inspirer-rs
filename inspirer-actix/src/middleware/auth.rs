//! 授权认证中间件
//!

use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use futures::future::{ok, Ready};
use std::pin::Pin;
use std::future::Future;
use std::task::{Poll, Context};
use actix_web::http::header::AUTHORIZATION;
use actix_web::http::HeaderValue;
use failure::_core::marker::PhantomData;
use jsonwebtoken::{decode, DecodingKey, Validation};

pub struct JwtTokenAuth<'a, K> {
    secret: &'a str,
    _phantom: PhantomData<K>,
}

impl<'a, S, B, K> Transform<S> for JwtTokenAuth<'a, K>
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
        S::Future: 'static,
        B: 'static,
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

pub struct JwtTokenAuthMiddleware<'a, S, K> {
    secret: &'a str,
    service: S,
    _phantom: PhantomData<K>,
}

impl<'a, S, B, K> Service for JwtTokenAuthMiddleware<'a, S, K>
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        match req.headers().get(AUTHORIZATION).map(HeaderValue::to_str) {
            Some(Ok(token_raw)) => if token_raw.starts_with("Bearer") {
                let split_result: Vec<&str> = token_raw.split(' ').collect();
                if split_result.len() == 2 {
                    let token_str = split_result[1];

                    // Decode
                    let token_decode = decode::<K>(
                        &token_str,
                        &DecodingKey::from_secret(self.secret.as_ref()),
                        &Validation::default()
                    );

                    match token_decode {
                        Ok(token) => {

                        }
                    }
                }
            }
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}