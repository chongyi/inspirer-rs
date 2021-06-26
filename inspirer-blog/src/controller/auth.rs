use actix_web::{get, post, HttpResponse};
use crate::service::auth::{AuthService, AuthTokenService};
use actix_web::web::Json;
use crate::error::Result;
use crate::model::user::UserSession;

#[derive(Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

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

#[get("/status")]
pub async fn status(session: UserSession) -> HttpResponse {
    HttpResponse::Ok().json(session)
}