use futures::Future;
use actix_web::*;

use state::AppState;
use util::message::{Pagination, CreatedObjectIdMessage};
use util::helper::get_paginate_params;
use models::category::{GetCategoryList, CreateCategory, NewCategory};

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