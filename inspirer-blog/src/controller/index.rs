use actix_web::{get, HttpResponse};
use crate::service::content::ContentService;
use crate::dao::content::{ContentQueryCondition, ContentQuerySort, Key};
use inspirer_actix_ext::database::statement::sort::{SortStatement, Sort};
use inspirer_actix_ext::database::statement::pagination::Paginate;
use crate::error::Result;

#[get("/")]
pub async fn home(srv: ContentService) -> Result<HttpResponse> {
    let mut sort = SortStatement::default();
    sort.push(Sort::Desc(ContentQuerySort::Id));

    Ok(HttpResponse::Ok()
        .json(srv.list(ContentQueryCondition {
            paginate: Some(Paginate::new(3)),
            sort,
            ..Default::default()
        }).await?))
}

#[get("/item")]
pub async fn item(srv: ContentService) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .json(srv.find(Key::Id(8)).await?))
}