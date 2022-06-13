use axum::extract::{FromRequest, RequestParts};
use chrono::{Duration, Utc};
use inspirer_content::util::uuid::Uuid;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::error::{InspirerError, InspirerResult};

const DEFAULT_TOKEN_EXPIRATION: i64 = 1440;

lazy_static! {
    static ref EXPIRATION: i64 = {
        std::env::var("TOKEN_EXPIRATION")
            .map(|v| v.parse::<i64>().expect("TOKEN_EXPIRATION 参数格式化错误"))
            .unwrap_or(DEFAULT_TOKEN_EXPIRATION)
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: u128,
    pub exp: usize,
    pub iat: usize,
}

impl Default for Claims {
    fn default() -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::minutes(*EXPIRATION);

        Claims {
            sub: 0,
            exp: exp.timestamp() as usize,
            iat: iat.timestamp() as usize,
        }
    }
}

impl From<inspirer_content::model::user::UserModel> for Claims {
    fn from(model: inspirer_content::model::user::UserModel) -> Self {
        Claims {
            sub: model.id.as_u128(),
            ..Default::default()
        }
    }
}

impl Claims {
    pub fn to_token(&self) -> InspirerResult<String> {
        let token = encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(
                std::env::var("SECRET")
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("SECRET")
                    .as_ref(),
            ),
        )
        .map_err(|err| {
            tracing::error!("Create token error: {err}");
            InspirerError::CreateTokenError
        })?;

        Ok(token)
    }

    pub fn from_token(token: &str) -> InspirerResult<Self> {
        let token = decode::<Self>(
            token,
            &DecodingKey::from_secret(
                std::env::var("SECRET")
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("SECRET")
                    .as_ref(),
            ),
            &Validation::default(),
        )
        .or(Err(InspirerError::ParseTokenError))?;

        Ok(token.claims)
    }

    pub fn to_session_info(&self) -> SessionInfo {
        SessionInfo {
            uuid: Uuid::from_u128(self.sub),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SessionInfo {
    uuid: Uuid,
}

impl SessionInfo {
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
}

#[async_trait::async_trait]
impl<B> FromRequest<B> for SessionInfo
where
    B: Send,
{
    type Rejection = InspirerError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        req.extensions()
            .get::<SessionInfo>()
            .cloned()
            .ok_or(InspirerError::Unauthorized)
    }
}
