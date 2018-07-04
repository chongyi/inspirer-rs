use futures::future::{Future, IntoFuture};
use actix_web::*;

use state::AppState;
use util::message::{Pagination, CreatedObjectIdMessage, DeletedObjectMessage, UpdateByID};
use util::error::{runtime_error_container, ApplicationLogicError};
use util::helper::get_paginate_params;
use models::category::{GetCategoryList, CreateCategory, NewCategory, DeleteCategory, UpdateCategory};

pub fn get_category_list(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let (page, per_page) = get_paginate_params(&req);
    let message = Pagination::new(page, per_page, Some(GetCategoryList {
        name: req.query().get("name").map(|r| r.to_string()),
    }));

    req.state().database.send(message).from_err().and_then(move |res| {
        Ok(HttpResponse::Ok().json(res.map_err(runtime_error_container(req).into())?))
    }).responder()
}

pub fn create_category(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    Form::<CreateCategory>::extract(&req).from_err().and_then(move |res| {
        req.state().database.send(NewCategory::from(res.into_inner())).from_err()
    }).and_then(|res| {
        Ok(HttpResponse::Ok().json(CreatedObjectIdMessage {
            id: res.map_err(runtime_error_container(req).into())?
        }))
    }).responder()
}

pub fn delete_category(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let match_info = match req.match_info().get("id") {
        Some(s) => Ok(s.parse::<u32>().unwrap()),
        None => Err(ApplicationLogicError::InvalidArgument)
    };

    match_info.into_future()
        .map_err(runtime_error_container(req.clone()).into())
        .and_then(move |res| {
            req.state().database.send(DeleteCategory(res)).map_err(runtime_error_container(req).into())
        })
        .and_then(|res| {
            Ok(HttpResponse::Ok().json(DeletedObjectMessage {
                count: res.map_err(runtime_error_container(req).into())?
            }))
        }).responder()
}

pub fn update_category(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let extract = req.clone();

    Form::<UpdateCategory>::extract(&req).from_err().and_then(move |update| {
        let match_info = match extract.match_info().get("id") {
            Some(s) => Ok(s.parse::<u32>().unwrap()),
            None => Err(ApplicationLogicError::InvalidArgument)
        };

        match_info.into_future()
            .map_err(error::ErrorBadRequest)
            .and_then(move |res| {
                req.state().database.send(UpdateByID::<UpdateCategory> {
                    id: res,
                    update: update.into_inner(),
                }).from_err()
            })
    }).and_then(|res| {
        Ok(HttpResponse::Ok().json(res.map_err(runtime_error_container(req).into())?))
    }).responder()
}