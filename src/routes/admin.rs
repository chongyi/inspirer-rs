use actix_web::{Scope, http::Method, HttpRequest, FromRequest};

use state::AppState;
use controllers::admin;
use middlewares::authenticate::Authenticate as MAuthenticate;

pub fn admin_routes<S>(scope: Scope<S>) -> Scope<S>
    where HttpRequest<AppState>: FromRequest<S>
{
    scope
        .route("/authentication", Method::POST, admin::authorization::authorization)
        .nested("", |scope| {
            scope.middleware(MAuthenticate)
                .route("/session/current-user", Method::GET, admin::user::get_current_user_info)
                .route("/category", Method::GET, admin::category::get_category_list)
                .route("/category", Method::POST, admin::category::create_category)
                .route("/category/{id:\\d+}", Method::DELETE, admin::category::delete_category)
                .route("/category/{id:\\d+}", Method::PUT, admin::category::update_category)
                .route("/category/{id:\\d+}", Method::GET, admin::category::get_category)
                .route("/content", Method::POST, admin::content::create_content)
                .route("/content", Method::GET, admin::content::get_content_list)
                .route("/content/{id:\\d+}", Method::GET, admin::content::get_content)
                .route("/content/{id:\\d+}", Method::DELETE, admin::content::delete_content)
                .route("/content/{id:\\d+}", Method::PUT, admin::content::update_content)
        })
}