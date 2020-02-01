use actix_web::{web};
use crate::controller::admin;

pub fn scoped_admin(cfg: &mut web::ServiceConfig) {
    cfg.service(admin::session::current_session_profile);
}