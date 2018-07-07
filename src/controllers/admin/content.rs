use actix_web::*;
use actix_web::middleware::session::RequestSession;
use futures::future::{IntoFuture, Future, ok as FutOk, err as FutErr};

use state::AppState;
use util::error::{error_handler, ApplicationError};
use util::message::CreatedObjectIdMessage;
use models::content::CreateContent;
use util::auth::PrivateClaims;

pub fn create_content(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let origin = req.clone();
    let claims = req.session().get::<PrivateClaims>("claims").unwrap().unwrap();
    Form::<CreateContent>::extract(&req).from_err()
        .and_then(move |res| {
            let mut data = res.into_inner();

            if let Some(uid) = data.creator_id {
                if uid != claims.uid {
                    return FutErr(ApplicationError::SysInvalidArgumentError());
                }
            } else {
                data.creator_id = Some(claims.uid);
            }

            FutOk(data)
        })
        .and_then(move |res| {
            req.state().database.send(res).from_err()
        })
        .and_then(|res| {
            Ok(HttpResponse::Ok().json(CreatedObjectIdMessage { id: res? }))
        })
        .map_err(error_handler(origin))
        .responder()
}