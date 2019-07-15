use actix_web::{web};
use crate::handler::admin;

pub fn scoped_admin(cfg: &mut web::ServiceConfig) {
    cfg.route("/content", web::get().to_async(admin::contents::get_contents));
}