use crate::{controller, middleware::auth::auth};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};

pub fn create_routes() -> Router {
    Router::new()
        .route(
            "/contents",
            get(controller::content::get_content_list_simple),
        )
        .route("/content/:id", get(controller::content::find_content))
        .route("/login", post(controller::auth::login))
        .nest("/security", secure_routes())
}

pub fn secure_routes() -> Router {
    Router::new()
        .route("/profile", get(controller::auth::get_profile))
        .route(
            "/content",
            get(controller::content::get_content_list).post(controller::content::create_content),
        )
        .route("/content/:id", get(controller::content::get_content))
        .route_layer(middleware::from_fn(auth))
}
