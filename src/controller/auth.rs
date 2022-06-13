use axum::{Extension, Json};
use inspirer_content::{manager::Manager, service::user::UserService, util::uuid::uuid_to_base62};

use crate::{
    error::{InspirerError, InspirerResult},
    request::auth::LoginPayload,
    response::auth::{AccessToken, UserProfile},
    session::{Claims, SessionInfo},
};

pub async fn login(
    Extension(manager): Extension<Manager>,
    Json(payload): Json<LoginPayload>,
) -> InspirerResult<Json<AccessToken>> {
    let user = manager.attempt(payload.username, payload.password).await?;

    Ok(Json(AccessToken {
        access_token: Claims::from(user).to_token()?,
    }))
}

pub async fn get_profile(
    Extension(manager): Extension<Manager>,
    session: SessionInfo,
) -> InspirerResult<Json<UserProfile>> {
    manager
        .get_user_by_id(session.uuid())
        .await?
        .ok_or(InspirerError::Unauthorized)
        .map(|model| UserProfile {
            id: uuid_to_base62(model.id),
            nickname: model.nickname,
            username: model.username,
            avatar: model.avatar,
        })
        .map(Json)
}
