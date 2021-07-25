use actix_web::{Error, FromRequest, HttpRequest};
use actix_web::dev::Payload;
use futures::future::{ok, Ready};

use inspirer_content_common::model::user::UserTokenPayload;

use crate::error::{Result, RuntimeError};

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