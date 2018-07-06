use actix_web::*;
use futures::future::{IntoFuture, Future};

use state::AppState;
use util::error::ApplicationError;
use util::message::CreatedObjectIdMessage;
use models::content::CreateContent;

pub fn create_content(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let origin = req.clone();
    Form::<CreateContent>::extract(&req).from_err()
        .and_then(move |res| {
            req.state().database.send(res.into_inner()).from_err()
        })
        .and_then(|res| {
            Ok(HttpResponse::Ok().json(CreatedObjectIdMessage { id: res? }))
        })
        .map_err(error_handler!(origin))
        .responder()
}