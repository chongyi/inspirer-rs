//! 授权认证中间件
//!

use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use futures::future::{ok, Ready};
use std::pin::Pin;
use std::future::Future;
use std::task::{Poll, Context};

pub(crate) struct Auth;

impl<S, B> Transform<S> for Auth
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

pub(crate) struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthMiddleware<S>
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}