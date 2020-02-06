use actix_web::HttpResponse;
use actix_web::Result;
use inspirer_actix::response::ResponseMessage;
use inspirer_actix::middleware::auth::TokenGuard;
use crate::middleware::auth::Credential;

#[get("/current-session/profile")]
pub async fn current_session_profile(token: TokenGuard<Credential>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(ResponseMessage::ok(token.get_token().unwrap())))
}