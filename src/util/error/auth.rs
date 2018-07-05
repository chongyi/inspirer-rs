use actix_web::http::StatusCode;

use super::ApplicationError;
use super::ErrorInformation;

impl ApplicationError {
    error_trigger_define!(ApplicationError::AuthenticationError, AUTH_VALIDATION, 10241, "Authentication invalidate.", StatusCode::FORBIDDEN, __, AuthValidationError);
}