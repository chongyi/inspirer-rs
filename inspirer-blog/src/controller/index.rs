use actix_web::{get, HttpResponse};
use actix_web::web::{Path, Query};
use inspirer_actix_ext::database::statement::pagination::Paginate;
use inspirer_actix_ext::database::statement::sort::{Sort, SortStatement};
use inspirer_actix_ext::validator::{Validate, Validated};

use crate::dao::content::{ContentQueryCondition, Key};
use crate::error::Result;
use crate::request::content::{ContentQuerySort, FindContent, ClientQueryContent};
use crate::service::content::ContentService;

#[get("/")]
pub async fn home(srv: ContentService) -> Result<HttpResponse> {
    let mut sort = SortStatement::default();
    sort.push(Sort::Desc(ContentQuerySort::Id));

    Ok(HttpResponse::Ok()
        .json(srv.list(ContentQueryCondition {
            paginate: Some(Paginate::default()),
            sort,
            ..Default::default()
        }).await?))
}

#[get("/content/{id}")]
pub async fn get_content(srv: ContentService, find: Validated<Path<FindContent>>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .json(srv.find(Key::Id(find.id)).await?))
}

#[get("/content")]
pub async fn get_content_list(srv: ContentService, query: Validated<Query<ClientQueryContent>>) -> Result<HttpResponse> {
    let mut sort = SortStatement::default();
    sort.push(Sort::Desc(ContentQuerySort::Id));
    sort.push(Sort::Desc(ContentQuerySort::PublishedAt));

    let condition = ContentQueryCondition {
        sort,
        is_display: Some(true),
        is_published: Some(true),
        paginate: Some(Paginate::new(query.page, query.per_page)),
        ..Default::default()
    };

    Ok(
        srv.list_for_client(condition)
            .await
            .map(|res| HttpResponse::Ok().json(res))?
    )
}