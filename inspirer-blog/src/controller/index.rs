use actix_web::{get, HttpResponse};
use crate::service::content::ContentService;
use crate::dao::content::{ContentQueryCondition, ContentQuerySort};
use crate::model::{Paginator, SortCondition, SortOption};

#[get("/")]
pub async fn home(srv: ContentService) -> HttpResponse {
    let mut sort = SortCondition::default();
    sort.push(SortOption::desc(ContentQuerySort::Id));
    HttpResponse::Ok()
        .json(srv.list(ContentQueryCondition {
            paginator: Some(Paginator {
                page: 1,
                per_page: 3,
            }),
            sort,
            ..Default::default()
        }).await)
}