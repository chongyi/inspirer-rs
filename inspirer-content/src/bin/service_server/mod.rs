use inspirer_content::ContentService;
use axum::extract::Extension;
use sqlx::MySqlPool;
use axum::prelude::*;
use serde::Deserialize;
use inspirer_content::model::ContentEntityWritable;
use axum::response::{IntoResponse, Json};
use serde_json::json;
use axum::http::{StatusCode, Response};

#[derive(Deserialize)]
pub struct CreateContent {
    author_id: u64,
    title: String,
    #[serde(default)]
    keywords: String,
    #[serde(default)]
    description: String,
    content: String,
}

pub async fn create(Extension(pool): Extension<MySqlPool>, extract::Json(content): extract::Json<CreateContent>) -> Response<Body> {
    let result = pool.create(content.author_id, ContentEntityWritable {
        title: content.title.as_str(),
        keywords: content.keywords.as_str(),
        description: content.description.as_str(),
        content: content.content.as_str(),
    }).await;

    match result {
        Ok(id) => Json(json!({
            "id": id
        })).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err)).into_response()
    }
}