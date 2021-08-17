use std::option::Option::Some;

use axum::extract::{ContentLengthLimit, Extension, Multipart, Path, Query};
use axum::http::{Response, StatusCode};
use axum::prelude::*;
use axum::response::{IntoResponse, Json};
use multer::parse_boundary;
use serde::Deserialize;
use serde_json::json;
use sqlx::MySqlPool;

use inspirer_content::ContentService;
use inspirer_content::model::{AdvanceContentQuery, ContentEntityWritable, SimpleContentQuery};
use inspirer_query_ext::model::{Paginate, PaginateWrapper};

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

pub async fn list_simple(Extension(pool): Extension<MySqlPool>, Query(paginate): Query<Paginate>) -> Response<Body> {
    let query = paginate.wrap(SimpleContentQuery::default().into());

    let result = pool.list(query)
        .await;

    match result {
        Ok(data) => Json(data).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err)).into_response()
    }
}

pub async fn find(Extension(pool): Extension<MySqlPool>, Path((id, )): Path<(u64, )>) -> Response<Body> {
    let result = pool.get(id).await;
    match result {
        Ok(data) => Json(data).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err)).into_response()
    }
}