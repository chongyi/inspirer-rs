use axum::{http::Request, middleware::Next, response::Response};

use crate::{
    error::{InspirerError, InspirerResult},
    session::Claims,
};

pub async fn auth<B>(mut req: Request<B>, next: Next<B>) -> InspirerResult<Response> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(InspirerError::Unauthorized)?
        .strip_prefix("Bearer ");

    match auth_header {
        Some(auth_header) => {
            let claims = extract_token_payload(auth_header)?;

            req.extensions_mut().insert(claims.to_session_info());

            Ok(next.run(req).await)
        }
        _ => Err(InspirerError::Unauthorized),
    }
}

fn extract_token_payload(token: &str) -> InspirerResult<Claims> {
    Claims::from_token(token)
}
