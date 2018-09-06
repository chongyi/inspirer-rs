use std::rc::Rc;
use actix_web::{HttpRequest, HttpResponse, AsyncResponder, Responder, HttpMessage};
use actix_web::http;
use futures::future::Future;
use rss::{ChannelBuilder, Channel, Item, ItemBuilder};
use url::Url;
use comrak::{markdown_to_html, ComrakOptions};

use state::AppState;
use message::{Pagination, PaginatedListMessage};
use template::get_site_setting;
use error::{Error, error_handler};
use models::content;

pub fn rss(req: HttpRequest<AppState>) -> impl Responder {
    let req_ref = Rc::new(req);
    let req_for_db = Rc::clone(&req_ref);
    let req_for_data = Rc::clone(&req_ref);
    let req_for_err = Rc::clone(&req_ref);
    let setting = get_site_setting();
    let mut channel: Channel = ChannelBuilder::default()
        .title("Inspirer blog article")
        .link(setting.site.home)
        .description("An RSS feed.")
        .build()
        .unwrap();

    let default_getter = content::GetContents {
        category: Some(content::CategoryMatcher::NotZero),
        content_type: Some(1),
        ..Default::default()
    };
    let default_content = Pagination::new(Some(1), Some(10), Some(default_getter));

    req_for_db.state().database.send(default_content).from_err()
        .and_then(move |res| {
            let pagination: PaginatedListMessage<content::ContentDisplay> = res?;
            let mut items: Vec<Item> = vec![];
            for content in &pagination.list {
                let target = content.name.clone().unwrap_or(content.id.to_string());
                let url = req_for_data.url_for("article", &[target.as_str()])
                    .or(Err(Error::internal_server_error(Some("[unknown]"), None)))?;

                let item: Item = ItemBuilder::default()
                    .title(Some(content.title.clone()))
                    .link(Some(url.into_string()))
                    .description(Some(markdown_to_html(&content.description, &ComrakOptions::default())))
                    .build()
                    .unwrap();

                items.push(item);
            }

            channel.set_items(items);
            Ok(HttpResponse::Ok().content_type("application/xml;charset=utf-8").body(channel.to_string()))
        })
        .map_err(error_handler(req_for_err))
        .responder()
}