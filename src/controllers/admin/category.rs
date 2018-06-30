use futures::future::{Future, result, IntoFuture};
use actix_web::*;
use actix_web::dev::FromParam;

use state::AppState;
use util::message::{Pagination, CreatedObjectIdMessage, RuntimeError, DeletedObjectMessage};
use util::helper::get_paginate_params;
use models::category::{GetCategoryList, CreateCategory, NewCategory, DeleteCategory};

pub fn get_category_list(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let (page, per_page) = get_paginate_params(&req);
    let message = Pagination::new(page, per_page, Some(GetCategoryList {
        name: req.query().get("name").map(|r| r.to_string()),
    }));

    req.state().database.send(message).from_err().and_then(|res| {
        Ok(HttpResponse::Ok().json(res?))
    }).responder()
}

pub fn create_category(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    Form::<CreateCategory>::extract(&req).from_err().and_then(move |res| {
        req.state().database.send(NewCategory::from(res.into_inner())).from_err()
    }).and_then(|res| {
        Ok(HttpResponse::Ok().json(CreatedObjectIdMessage {
            id: res?
        }))
    }).responder()
}

pub fn delete_category(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let sender = req.clone();
    let match_info = match req.match_info().get("id") {
        Some(s) => Ok(s.to_string()),
        None => Err(RuntimeError::InvalidArgument)
    };

    match_info.into_future()
        .map_err(error::ErrorBadRequest)
        .and_then( |res| {
            result(res.parse::<u32>()).map_err(error::ErrorBadRequest)
        })
        .and_then(move |res| {
            sender.state().database.send(DeleteCategory(res)).map_err(error::ErrorInternalServerError)
        })
        .and_then(|res| {
            Ok(HttpResponse::Ok().json(DeletedObjectMessage {
                count: res?
            }))
        }).responder()
}