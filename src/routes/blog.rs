use actix_web::{Scope, http::Method, HttpRequest, FromRequest};

use state::AppState;

//pub fn blog_routes<S>(scope: Scope<S>) -> Scope<S>
//    where HttpRequest<AppState>: FromRequest<S>
//{
//}