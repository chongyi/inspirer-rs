use crate::{controller, middleware::auth::auth};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};

pub fn create_routes() -> Router {
    Router::new()
        .route("/contents", get(controller::content::get_content_list))
        .route("/content/:id", get(controller::content::find_content))
        .route("/login", post(controller::auth::login))
        .route(
            "/profile",
            get(controller::auth::get_profile).layer(middleware::from_fn(auth)),
        )
}
