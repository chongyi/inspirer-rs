use std::ops::Deref;
use actix_web::{FromRequest, HttpRequest, Error};
use std::future::Future;
use actix_web::dev::Payload;
use futures::future::{Ready, ok};
use crate::error::{Result, RuntimeError};

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::FromRow)]
pub struct UserBasic {
    pub id: u64,
    pub user_type: u16,
    pub username: String,
    pub nickname: String,
    pub avatar: String,
    #[serde(skip_serializing)]
    pub password: String,
}

/// 用于用户 Json Web Token 的模型
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct UserTokenPayload {
    pub id: u64,
    pub user_type: u16,
}


impl From<&UserBasic> for UserTokenPayload {
    fn from(user_basic: &UserBasic) -> Self {
        UserTokenPayload {
            id: user_basic.id,
            user_type: user_basic.user_type,
        }
    }
}

#[derive(Serialize)]
pub struct UserSession {
    #[serde(flatten)]
    inner: Option<UserTokenPayload>,
}

impl FromRequest for UserSession {
    type Error = Error;
    type Future = Ready<Result<UserSession, Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ok(
            UserSession {
                inner: req.extensions().get::<UserTokenPayload>().cloned()
            }
        )
    }
}

impl UserSession {
    pub fn is_login(&self) -> bool {
        !self.inner.is_none()
    }

    pub fn inner(&self) -> Option<&UserTokenPayload> {
        self.inner.as_ref()
    }

    pub fn user_id(&self) -> Result<u64> {
        self.inner
            .map(|inner| inner.id)
            .ok_or(RuntimeError::InvalidToken.into())
    }
}