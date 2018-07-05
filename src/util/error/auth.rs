use actix_web::http::StatusCode;

use super::ApplicationError;
use super::ErrorInformation;

impl ApplicationError {
    pub const AUTH_VALIDATION: (u16, &'static str, StatusCode) = (10241, "Authentication invalidate.", StatusCode::FORBIDDEN);

    #[allow(non_snake_case)]
    pub fn AuthValidationError() -> Self {
        let (a, b, c) = Self::AUTH_VALIDATION;
        ApplicationError::AuthenticationError(ErrorInformation::new(
            a, b.into(), c, None
        ))
    }
}