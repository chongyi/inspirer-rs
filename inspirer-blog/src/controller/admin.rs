use actix_web::{HttpResponse, get, post, put, delete, HttpRequest};
use actix_web::web::{Json, Path};
use inspirer_actix_ext::validator::Validated;
use serde_json::json;

use crate::error::Result;
use crate::model::user::UserSession;
use crate::request::content::{CreateContent, DeleteOption, AdminQueryContent, ContentQuerySort};
use crate::service::content::ContentService;
use crate::dao::content::ContentQueryCondition;
use inspirer_actix_ext::database::statement::pagination::Paginate;
use inspirer_actix_ext::database::statement::sort::Sort;
use serde_qs::actix::QsQuery;

#[post("/admin/content")]
pub async fn create_content(
    session: UserSession,
    create_content: Validated<Json<CreateContent>>,
    srv: ContentService,
) -> Result<HttpResponse> {
    let id = srv.create_content_simple(session.user_id()?, &create_content)
        .await?;

    Ok(HttpResponse::Ok().json(json!({
        "id": id
    })))
}

#[put("/admin/content/{id}")]
pub async fn update_content(
    Path((content_id, )): Path<(u64, )>,
    session: UserSession,
    update_content: Validated<Json<CreateContent>>,
    srv: ContentService,
) -> Result<HttpResponse> {
    let result = srv.update_content_simple(session.user_id()?, content_id, &update_content)
        .await?;

    Ok(HttpResponse::Ok().json(json!({
        "result": result
    })))
}

#[delete("/admin/content/{id}")]
pub async fn delete_content(
    Path((content_id, )): Path<(u64, )>,
    query: QsQuery<DeleteOption>,
    srv: ContentService,
) -> Result<HttpResponse> {
    let result = if query.force {
        srv.force_delete_content_simple(content_id)
            .await?
    } else {
        srv.delete_content_simple(content_id)
            .await?
    };

    Ok(HttpResponse::Ok().json(json!({
        "result": result
    })))
}

#[get("/admin/content")]
pub async fn get_content_list(
    req: HttpRequest,
    query: Validated<QsQuery<AdminQueryContent>>,
    srv: ContentService,
) -> Result<HttpResponse> {
    debug!("{}", req.query_string());

    let query = query.0.into_inner();
    let condition = ContentQueryCondition {
        is_deleted: Some(query.is_deleted),
        is_published: query.is_published,
        is_display: query.is_display,
        paginate: Some(Paginate::new(query.page, query.per_page)),
        sort: query.sorts.unwrap_or(vec![Sort::Desc(ContentQuerySort::CreatedAt)]),
        ..Default::default()
    };

    debug!("{:?}", condition);

    srv.list(condition)
        .await
        .map_err(Into::into)
        .map(|res| HttpResponse::Ok().json(res))
}