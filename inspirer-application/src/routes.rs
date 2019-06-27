use actix_web::{web};
use crate::handler::admin;

pub fn scoped_admin(cfg: &mut web::ServiceConfig) {
    cfg.service(admin::contents::get_contents);
}