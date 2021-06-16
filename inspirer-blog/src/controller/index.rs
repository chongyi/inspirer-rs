use actix_web::{get, HttpResponse};
use crate::service::content::ContentService;
use crate::dao::content::{ContentQueryCondition, ContentQuerySort};
use inspirer_actix_ext::database::statement::sort::{SortStatement, Sort};
use inspirer_actix_ext::database::statement::pagination::Paginate;

#[get("/")]
pub async fn home(srv: ContentService) -> HttpResponse {
    let mut sort = SortStatement::default();
    sort.push(Sort::Desc(ContentQuerySort::Id));

    HttpResponse::Ok()
        .json(srv.list(ContentQueryCondition {
            paginate: Some(Paginate::new(3)),
            sort,
            ..Default::default()
        }).await)
}