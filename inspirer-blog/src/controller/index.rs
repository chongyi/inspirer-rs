use actix_web::{get, HttpResponse};
use crate::service::content::ContentService;
use crate::dao::content::{ContentQueryCondition, ContentQuerySort, Key};
use inspirer_actix_ext::database::statement::sort::{SortStatement, Sort};
use inspirer_actix_ext::database::statement::pagination::Paginate;
use crate::error::Result;
use inspirer_actix_ext::validator::{Validated, Validate};
use actix_web::web::Path;

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

#[derive(Deserialize, Validate)]
pub struct FindContent {
    #[validate(range(min = 1))]
    id: u64
}

#[get("/content/{id}")]
pub async fn item(srv: ContentService, find: Validated<Path<FindContent>>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .json(srv.find(Key::Id(find.id)).await?))
}