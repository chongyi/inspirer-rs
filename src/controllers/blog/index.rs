use super::{PushMessage, Content};

use std::rc::Rc;
use futures::future::{Future, ok as FutOk, err as FutErr};
use actix_web::{HttpRequest, HttpResponse, Responder, AsyncResponder, HttpMessage};
use comrak::{markdown_to_html, ComrakOptions};
use tera::Context;

use state::AppState;
use message::Pagination;
use models::{push_message, content, recommend};
use template::{get_global_context, TEMPLATES};
use error::error_handler;

pub fn home(req: HttpRequest<AppState>) -> impl Responder {
    let ref_req = Rc::new(req);
    let req_for_contents = Rc::clone(&ref_req);
    let req_for_pushes = Rc::clone(&ref_req);
    let req_for_recommends = Rc::clone(&ref_req);
    let req_for_err = Rc::clone(&ref_req);

    let default_content = Pagination {
        page: 1,
        per_page: 10,
        filter: Some(content::GetContents::default()),
    };

    let default_pushes = Pagination {
        page: 1,
        per_page: 3,
        filter: Some(push_message::GetPushMessages::default()),
    };

    let default_recommends = recommend::GetRecommendContents::default();

    req_for_contents.state().database.send(default_content).from_err()
        .and_then(move |contents| {
            req_for_pushes.state().database.send(default_pushes).from_err()
                .and_then(move |pushes| {
                    req_for_recommends.state().database.send(default_recommends).from_err()
                        .and_then(move |recommends| {
                            Ok((contents?, pushes?, recommends?))
                        })
                })
        })
        .and_then(|res| {
            let (contents, push_messages, mut recommends) = res;
            let mut list: Vec<Content> = vec![];
            let mut pushes: Vec<PushMessage> = vec![];

            for item in contents.list {
                list.push(Content {
                    id: item.id,
                    name: item.name,
                    title: item.title,
                    description: item.description,
                    published_at_o: item.published_at,
                    published_at: item.published_at.map(|v| v.format("%Y-%m-%d").to_string()),
                });
            }

            for item in push_messages.list {
                pushes.push(PushMessage {
                    id: item.id,
                    content: item.content,
                    created_at: item.created_at.format("%Y-%m-%d").to_string(),
                    created_at_o: item.created_at,
                })
            }

            let recommends: Vec<recommend::RecommendContentDisplay> = recommends
                .iter_mut()
                .map(|item| {
                    item.summary = markdown_to_html(&item.summary, &ComrakOptions::default());
                    item.clone()
                })
                .collect();

            let mut context = Context::new();
            context.add("contents", &list);
            context.add("pushes", &pushes);
            context.add("recommends", &recommends);
            context.extend(get_global_context());

            let rendered = match TEMPLATES.render("home.html", &context) {
                Ok(r) => r,
                Err(e) => "Render error".into()
            };
            Ok(HttpResponse::Ok().body(rendered))
        })
        .map_err(error_handler(req_for_err))
        .responder()
}
