use actix_web::{Scope, http::Method, HttpRequest, FromRequest};

use state::AppState;
use controllers::blog;

pub fn blog_routes<S: 'static>(scope: Scope<S>) -> Scope<S>
    where HttpRequest<AppState>: FromRequest<S>
{
    scope
        .route("/", Method::GET, blog::index::home)
        .route("/article", Method::GET, blog::content::article_list)
        .route("/article/{name}", Method::GET, blog::content::content)
        .route("/push", Method::GET, blog::push_message::push_message_list)
        .route("/feed", Method::GET, blog::rss::rss)
        .route("/{name}", Method::GET, blog::content::source)
}