use std::task::{Context, Poll};

use actix::Addr;
use actix::fut::ready;
use actix_http::HttpMessage;
use actix_redis::{Command, RedisActor, RespValue};
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::HttpResponse;
use futures::future::{Either, ok, Ready};
use redis_async::resp_array;

use inspirer_actix::error::{ActixErrorWrapper, map_to_inspirer_response_err, UNAUTHORIZED, CodedError};
use inspirer_actix::middleware::auth::TokenGuard;
use inspirer_actix::response::ResponseMessage;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Serialize, Deserialize)]
pub struct Credential {
    pub uuid: String,
    pub exp: usize,
}

pub struct Auth;

impl<S, B> Transform<S> for Auth
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=actix_web::Error> + 'static,
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
        ok(AuthMiddleware { service: Rc::new(RefCell::new(service)) })
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<RefCell<S>>
}

impl<S, B> Service for AuthMiddleware<S>
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=actix_web::Error> + 'static,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;
//    type Future = Pin<Box<dyn Future<Output = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>>>>;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let mut service = self.service.clone();

        Box::pin(async move {
            let valid = match req.extensions().get::<TokenGuard<Credential>>() {
                Some(token) => {
                    if let Some(redis_actor) = req.app_data::<Addr<RedisActor>>() {
                        let result = redis_actor
                            .send(Command(resp_array!("GET", format!("blocked:token:{}", token.get_origin_token()))))
                            .await
                            .map_err(map_to_inspirer_response_err(&req))??;

                        match result {
                            RespValue::Nil => true,
                            _ => false,
                        }
                    } else {
                        true
                    }
                }
                None => false
            };

            if valid {
                service.call(req).await
            } else {
                Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(ResponseMessage::<Option<bool>> {
                            code: UNAUTHORIZED.0,
                            msg: UNAUTHORIZED.1,
                            data: &None,
                        })
                        .into_body()
                ))
            }
        })
    }
}