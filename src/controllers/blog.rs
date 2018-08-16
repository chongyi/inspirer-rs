use std::rc::Rc;
use actix_web::{HttpRequest, HttpResponse, Responder, AsyncResponder};
use state::AppState;
use futures::{Future, future::err as FutErr};
use tera::{Tera, Context};
use template::TEMPLATES;
use models::content;
use message::{PaginatedListMessage, Pagination};
use error::error_handler;
use chrono::NaiveDateTime;
use error::Error;
use regex;
use comrak::{markdown_to_html, ComrakOptions};

pub fn home(req: HttpRequest<AppState>) -> impl Responder {
    let ref_req = Rc::new(req);
    let req_for_contents = Rc::clone(&ref_req);
    let req_for_err = Rc::clone(&ref_req);
    let default_content = Pagination {
        page: 1,
        per_page: 10,
        filter: Some(content::GetContents::default())
    };

    req_for_contents.state().database.send(default_content).from_err()
        .and_then(|contents| {
            #[derive(Serialize)]
            struct Content {
                id: u32,
                name: Option<String>,
                title: String,
                published_at_o: Option<NaiveDateTime>,
                published_at: Option<String>
            };

            let contents: PaginatedListMessage<content::ContentDisplay> = contents?;
            let mut list: Vec<Content> = vec![];

            for item in contents.list {
                list.push(Content {
                    id: item.id,
                    name: item.name,
                    title: item.title,
                    published_at_o : item.published_at,
                    published_at: item.published_at.map(|v| v.format("%Y-%m-%d").to_string())
                });
            }

            let mut context = Context::new();
            context.add("contents", &list);
            let rendered = match TEMPLATES.render("home.html", &context) {
                Ok(r) => r,
                Err(e) => format!(
                    "<p><strong>Error</strong>({}) {}</p>\n<p><pre>{:?}</pre></p>",
                    123,
                    "fuck",
                    e
                )
            };
            Ok(HttpResponse::Ok().body(rendered))
        })
        .map_err(error_handler(req_for_err))
        .responder()
}

pub fn content_list(req: HttpRequest<AppState>) -> impl Responder {
    "🚧🚧🚧"
}

pub fn content(req: HttpRequest<AppState>) -> impl Responder {
    let ref_req = Rc::new(req);
    let req_for_contents = Rc::clone(&ref_req);
    let req_for_err = Rc::clone(&ref_req);
    let name = match req_for_contents.match_info().get("name") {
        Some(name) => {
            let numeric = regex::Regex::new(r"^\d+$").unwrap();
            let name_string = regex::Regex::new(r"^\w+(-\w+)*$").unwrap();

            if numeric.is_match(name) {
                let id = name.parse::<u32>().unwrap();
                content::FindContent::ById(id)
            } else if name_string.is_match(name) {
                content::FindContent::ByName(name.into())
            } else {
                return FutErr(error_handler(req_for_err)(Error)).responder();
            }
        },
        None => return FutErr(error_handler(req_for_err)(Error)).responder(),
    };

    req_for_contents.state().database.send(name).from_err()
        .and_then(|res| {
            #[derive(Serialize)]
            struct Content {
                title: String,
                content: String,
                published_at_o: Option<NaiveDateTime>,
                published_at: Option<String>
            }

            let origin: content::ContentFullDisplay = res?;
            let data = Content {
                content: markdown_to_html(&origin.content.unwrap_or(String::from("")), &ComrakOptions::default()),
                title: origin.title,
                published_at_o : origin.published_at,
                published_at: origin.published_at.map(|v| v.format("%Y-%m-%d").to_string())
            };

            let mut context = Context::new();
            context.add("content", &data);
            let rendered = match TEMPLATES.render("content.html", &context) {
                Ok(r) => r,
                Err(e) => format!(
                    "<p><strong>Error</strong>({}) {}</p>\n<p><pre>{:?}</pre></p>",
                    123,
                    "fuck",
                    e
                )
            };
            Ok(HttpResponse::Ok().body(rendered))
        })
        .map_err(error_handler(req_for_err))
        .responder()
}

pub fn page(req: HttpRequest<AppState>) -> impl Responder {
    "🚧🚧🚧"
}

pub fn push_message_list(req: HttpRequest<AppState>) -> impl Responder {
    "🚧🚧🚧"
}

pub fn push_message(req: HttpRequest<AppState>) -> impl Responder {
    "🚧🚧🚧"
}

pub fn subject_list(req: HttpRequest<AppState>) -> impl Responder {
    "🚧🚧🚧"
}

pub fn subject(req: HttpRequest<AppState>) -> impl Responder {
    "🚧🚧🚧"
}