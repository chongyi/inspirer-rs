use actix_web::{HttpResponse, post, put, delete};
use actix_web::web::{Json, Path, Query};
use inspirer_actix_ext::validator::Validated;
use serde_json::json;

use crate::error::Result;
use crate::model::user::UserSession;
use crate::request::content::{CreateContent, DeleteOption};
use crate::service::content::ContentService;

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
    Path((content_id,)): Path<(u64, )>,
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
    Path((content_id,)): Path<(u64, )>,
    query: Query<DeleteOption>,
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