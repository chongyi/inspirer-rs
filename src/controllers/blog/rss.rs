use actix_web::{HttpRequest, HttpResponse, AsyncResponder, Responder, HttpMessage};
use actix_web::http;
use futures::future::Future;
use rss::{ChannelBuilder, Channel, Item};
use url::Url;

use state::AppState;
use template::get_site_setting;

pub fn rss(req: HttpRequest<AppState>) -> impl Responder {
    let setting = get_site_setting();
    let channel: Channel = ChannelBuilder::default()
        .title("Inspirer blog article")
        .link(setting.site.home)
        .description("An RSS feed.")
        .build()
        .unwrap();

    HttpResponse::Ok().content_type("application/xml;charset=utf-8").body(channel.to_string())
}