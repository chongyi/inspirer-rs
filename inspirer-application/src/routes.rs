use actix_web::web;
use crate::controller::admin;
use crate::controller::credential;
use crate::middleware::auth::Auth;

pub fn inspirer_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/api/admin").wrap(Auth).configure(scoped_admin))
        .service(web::scope("/api").configure(scoped_common));
}

pub fn scoped_common(cfg: &mut web::ServiceConfig) {
    cfg.service(credential::create_credential);
}

pub fn scoped_admin(cfg: &mut web::ServiceConfig) {
    cfg.service(admin::session::current_session_profile);
}