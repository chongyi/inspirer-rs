use crate::controller;
use axum::{routing::get, Router};

pub fn create_routes() -> Router {
    Router::new()
        .route("/contents", get(controller::content::get_content_list))
        .route("/content/:id", get(controller::content::find_content))
}
