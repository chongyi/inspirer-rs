use actix_web::get;
use crate::service::content::ContentService;

#[get("/")]
pub async fn home(srv: ContentService) -> &'static str {
    srv.echo()
}