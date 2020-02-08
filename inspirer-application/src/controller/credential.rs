use actix_web::{web, HttpRequest, HttpResponse};
use inspirer_data_provider::prelude::{ConnPoolManager, ActiveModel};
use inspirer_data_provider::agent::user::{FindUserEmailCredential, FindUserMobilePhoneCredential, UserLoginTrigger};
use inspirer_data_provider::result::{ErrorKind, NotFoundError, ForbiddenRequestError, AuthenticationFailedError};
use inspirer_actix::error::map_to_inspirer_response_err;
use actix_service::Service;
use std::future::Future;
use inspirer_data_provider::utils::password_validate;
use jsonwebtoken::{Header, EncodingKey, Algorithm, encode};
use crate::middleware::auth::Credential;
use inspirer_actix::response::ResponseMessage;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum LoginCredential {
    MobilePhone { country_code: String, mobile_phone: String, },
    Email { email: String, },
}

#[derive(Deserialize)]
#[serde(tag = "type", content="val")]
pub enum Passphrase {
    #[serde(alias = "std")]
    StdPassword(String),
    #[serde(alias = "vc")]
    ValidateCode(String),
}

#[derive(Deserialize)]
pub struct LoginData {
    pub credential: LoginCredential,
    pub passphrase: Passphrase,
}

#[post("/session-credential")]
pub async fn create_credential(
    req: HttpRequest,
    db: web::Data<ConnPoolManager>,
    login_data: web::Json<LoginData>
) -> Result<HttpResponse, actix_web::Error> {
    let ip = req.peer_addr();

    web::block(move || {
        let conn = db.read().get().map_err(ErrorKind::from)?;

        let base_credential = match &login_data.credential {
            LoginCredential::Email { email } => FindUserEmailCredential { email: email.as_str(), status: None }
                .activate(&conn)?.map(|(_, base_credential)| base_credential),
            LoginCredential::MobilePhone { country_code, mobile_phone } => {
                FindUserMobilePhoneCredential {
                    country_code: country_code.as_str(),
                    mobile_phone: mobile_phone.as_str(),
                    status: None
                }.activate(&conn)?.map(|(_, base_credential)| base_credential)
            }
        };

        base_credential.ok_or(ErrorKind::biz_err(NotFoundError))
            .and_then(|base_credential| {
                match &login_data.passphrase {
                    Passphrase::ValidateCode(code) => Err(ErrorKind::biz_err(ForbiddenRequestError)),
                    Passphrase::StdPassword(password) => {
                        if password_validate(
                            password,
                            base_credential.password
                                .as_ref()
                                .map(|pwd| pwd.as_str())
                                .unwrap_or("")
                        ) {
                            let token = encode(
                                &Header::new(Algorithm::HS256),
                                &Credential {
                                    uuid: base_credential.uuid.clone(),
                                    exp: (chrono::Utc::now() + chrono::Duration::minutes(5)).timestamp() as usize,
                                },
                                &EncodingKey::from_secret("secret".as_ref()),
                            ).unwrap();

                            UserLoginTrigger {
                                user_uuid: base_credential.uuid.as_str(),
                                ip: ip.map(|v| format!("{}", v))
                                    .as_ref()
                                    .map(|v| v.as_str()),
                                event_time: None
                            }.activate(&conn)?;

                            Ok(token)
                        } else {
                            Err(ErrorKind::biz_err(AuthenticationFailedError))
                        }
                    },
                }
            })
    }).await
        .map_err(map_to_inspirer_response_err(&req))
        .map(|r| HttpResponse::Ok().json(ResponseMessage::ok(&r)))
}


//#[delete("/session-credential")]
//pub async fn delete_credential() {
//
//}