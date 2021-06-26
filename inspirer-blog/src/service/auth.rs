use anyhow::Result;
use inspirer_actix_ext::config::Config;
use inspirer_actix_ext::database::sqlx::MySqlPool;
use inspirer_actix_ext::service::{DependencyFactory, IntoService};

use crate::dao::user::Key;
use crate::error::RuntimeError::{PasswordVerifiedError, UserIsNotExists};
use crate::model::user::{UserBasic, UserTokenPayload};
use crate::service::user::UserService;
use crate::config::ApplicationConfig;
use chrono::{Utc, Duration};
use inspirer_json_web_token::{Claims, PublicClaims, EncodingKey, decode, DecodingKey, Validation};

#[derive(Service, FromRequest)]
pub struct AuthService {
    pool: MySqlPool,
    config: Config,
}

impl AuthService {
    pub async fn attempt(&self, username: &str, password: &str) -> Result<UserBasic> {
        let user = UserService::new(self.pool.clone())
            .get_user_basic_optional(Key::Username(username))
            .await?;

        match user {
            Some(user) => {
                if user.password.is_empty() {
                    Ok(user)
                } else {
                    Ok(pwhash::bcrypt::verify(password, user.password.as_str())
                        .then(|| user)
                        .ok_or(PasswordVerifiedError)?)
                }
            }
            None => Err(UserIsNotExists)?
        }
    }

    pub fn login(&self, user_basic: &UserBasic) -> Result<String> {
        // 此处接口目前仅支持 token 生成
        let app_config = self.config.get::<ApplicationConfig>("app")?;
        let claims = Claims::<UserTokenPayload> {
            public_claims: PublicClaims {
                iss: Some(app_config.domain.clone()),
                exp: (Utc::now().timestamp() + app_config.token_lifetime) as usize,
                ..Default::default()
            },
            private_claims: user_basic.into(),
        };

        Ok(claims.build_jwt_token()
            .encode_key(EncodingKey::from_secret(app_config.secret.as_bytes()))
            .build()?)
    }

    pub fn extract(&self, token: &str) -> Result<UserTokenPayload> {
        let app_config = self.config.get::<ApplicationConfig>("app")?;
        let result = decode::<Claims<UserTokenPayload>>(
            token,
            &DecodingKey::from_secret(app_config.secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(result.claims.private_claims)
    }
}