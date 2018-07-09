use actix_web::*;
use actix_web::middleware::session::RequestSession;
use futures::future::{IntoFuture, Future, ok as FutOk, err as FutErr};

use state::AppState;
use util::error::{error_handler, ApplicationError};
use util::message::{Pagination, CreatedObjectIdMessage, DeletedObjectMessage, UpdateByID};
use models::content::{CreateContent, GetContentList, FindContent, DeleteContent, PreUpdateContent};
use util::auth::PrivateClaims;
use util::helper::get_paginate_params;

pub fn create_content(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let origin = req.clone();
    let claims = req.session().get::<PrivateClaims>("claims").unwrap().unwrap();
    Json::<CreateContent>::extract(&req).from_err()
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

pub fn get_content_list(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let origin = req.clone();
    Query::<GetContentList>::extract(&req).into_future().from_err()
        .and_then(move |res| {
            let (page, per_page) = get_paginate_params(&req);
            req.state().database.send(Pagination::new(page, per_page, Some(res.into_inner()))).from_err()
        })
        .and_then(|res| {
            Ok(HttpResponse::Ok().json(res?))
        })
        .map_err(error_handler(origin))
        .responder()
}

pub fn get_content(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let origin = req.clone();
    let match_info = match req.match_info().get("id") {
        Some(s) => Ok(s.parse::<u32>().unwrap()),
        None => Err(ApplicationError::SysInvalidArgumentError())
    };

    match_info.into_future()
        .from_err()
        .and_then(move |res| {
            req.state().database.send(FindContent(res)).from_err()
        })
        .and_then(|res| {
            Ok(HttpResponse::Ok().json(res?))
        })
        .map_err(error_handler(origin))
        .responder()
}

pub fn delete_content(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let origin = req.clone();
    let match_info = match req.match_info().get("id") {
        Some(s) => Ok(s.parse::<u32>().unwrap()),
        None => Err(ApplicationError::SysInvalidArgumentError())
    };

    match_info.into_future()
        .from_err()
        .and_then(move |res| {
            req.state().database.send(DeleteContent(res)).from_err()
        })
        .and_then(|res| {
            Ok(HttpResponse::Ok().json(
                DeletedObjectMessage {
                    count: res?
                }
            ))
        })
        .map_err(error_handler(origin))
        .responder()
}

pub fn update_content(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let extract = req.clone();
    let origin = req.clone();

    Json::<PreUpdateContent>::extract(&req).from_err()
        .and_then(move |update| {
            let match_info = match extract.match_info().get("id") {
                Some(s) => Ok(s.parse::<u32>().unwrap()),
                None => Err(ApplicationError::SysInvalidArgumentError())
            };

            match_info.into_future()
                .from_err()
                .and_then(move |res| {
                    req.state().database.send(UpdateByID::<PreUpdateContent> {
                        id: res,
                        update: update.into_inner(),
                    }).from_err()
                })
        })
        .and_then(|res| {
            Ok(HttpResponse::Ok().json(res?))
        })
        .map_err(error_handler(origin))
        .responder()
}