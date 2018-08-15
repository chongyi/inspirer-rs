use actix_web::{Scope, http::Method, HttpRequest, FromRequest};

use state::AppState;
use controllers::blog;

pub fn blog_routes<S: 'static>(scope: Scope<S>) -> Scope<S>
    where HttpRequest<AppState>: FromRequest<S>
{
    scope
        .route("/", Method::GET, blog::home)
}