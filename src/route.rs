use crate::{controller, middleware::auth::auth};
use axum::{
    middleware,
    routing::{delete, get, post},
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
            "/content-service-config",
            get(controller::content::get_config),
        )
        .route(
            "/content",
            get(controller::content::get_content_list).post(controller::content::create_content),
        )
        .route(
            "/deleted/content",
            get(controller::content::get_deleted_content_list),
        )
        .route(
            "/deleted/content/:id",
            delete(controller::content::revert_deleted_content),
        )
        .route(
            "/content/:id",
            get(controller::content::get_content)
                .put(controller::content::update_content)
                .delete(controller::content::delete_content),
        )
        .route(
            "/content/:id/publish",
            post(controller::content::publish_content)
                .delete(controller::content::unpublish_content),
        )
        .route_layer(middleware::from_fn(auth))
}
