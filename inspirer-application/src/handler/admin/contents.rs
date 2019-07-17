use actix_web::web::{self, get, post, put, delete};
use actix_web::error::Error;
use futures::{Future, IntoFuture};
use actix_web::{HttpResponse, Responder, HttpRequest};
use inspirer_data_provider::agent::content::{GetContentsIndex, GetContent};
use crate::app::State;
use inspirer_data_provider::agent::ActiveModel;
use inspirer_data_provider::model::content::{ContentBase, ContentFull};
use inspirer_data_provider::model::user::BeJoinedUserBase;
use inspirer_data_provider::result::PaginateWrapper;
use crate::result::{InspirerResp, ResponseMessage};
use inspirer_actix::error::map_to_inspirer_response_err;
use regex::Regex;
use inspirer_data_provider::model::content_entity::ContentEntity;

#[get("/content")]
pub fn get_contents(params: Option<web::Query<GetContentsIndex>>, state: web::Data<State>, req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let pool = state.db_conn.read().clone();
    Box::new(
        web::block(move || {
            match params {
                Some(params) => params.activate(&pool.get().unwrap()),
                None => GetContentsIndex::default().activate(&pool.get().unwrap()),
            }
        })
            .and_then(|r| {
                #[derive(Serialize)]
                struct Item<'i> {
                    content: &'i ContentBase,
                    creator: &'i BeJoinedUserBase,
                }

                let data = r.list.iter().map(|(content, creator)| {
                    Item { content, creator }
                }).collect();

                Ok(HttpResponse::Ok().json(&ResponseMessage::ok(&PaginateWrapper {
                    list: data,
                    last_page: r.last_page,
                    total: r.total,
                })))
            })
            .map_err(map_to_inspirer_response_err(&req))
    )
}

#[get("/content/{id}")]
pub fn get_content(path: web::Path<String>, state: web::Data<State>, req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let pool = state.db_conn.read().clone();
    Box::new(
        web::block(move || {
            let active = if Regex::new(r"^\d+$").unwrap().is_match(path.as_str()) {
                GetContent::ById(path.parse::<i64>().unwrap())
            } else {
                GetContent::ByName(path.as_str())
            };

            active.activate(&pool.get().unwrap())
        })
            .and_then(|r| {
                #[derive(Serialize)]
                struct Data {
                    body: ContentFull,
                    entity: ContentEntity,
                    creator: BeJoinedUserBase,
                }

                Ok(HttpResponse::Ok().json(&ResponseMessage::ok(&Data {
                    body: r.0,
                    entity: r.1,
                    creator: r.2
                })))
            })
            .map_err(map_to_inspirer_response_err(&req))
    )
}