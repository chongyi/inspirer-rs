use actix_web::{get, HttpResponse, post};
use actix_web::web::Json;

use crate::error::Result;
use crate::model::user::UserSession;
use crate::request::auth::LoginPayload;
use crate::service::auth::{AuthService, AuthTokenService};
use crate::service::user::UserService;
use crate::dao::user::Key;

#[derive(Serialize)]
struct LoginResult {
    token: String,
}

#[post("/login")]
pub async fn login(auth: AuthService, auth_token: AuthTokenService, payload: Json<LoginPayload>) -> Result<HttpResponse> {
    let user = auth.attempt(payload.username.as_str(), payload.password.as_str())
        .await?;

    let token = auth_token.login(&user)?;
    Ok(HttpResponse::Ok().json(LoginResult { token }))
}

#[get("/current/user-info")]
pub async fn current_user_info(session: UserSession, srv: UserService) -> Result<HttpResponse> {
    srv.get_user_basic(Key::Id(session.user_id()?))
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(Into::into)
}