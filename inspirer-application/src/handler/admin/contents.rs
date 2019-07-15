use actix_web::web::{self, get, post, put, delete};
use actix_web::error::{Error};
use futures::{Future, IntoFuture};
use actix_web::{HttpResponse, Responder};
use inspirer_data_provider::agent::content::GetContentsIndex;
use crate::app::State;
use inspirer_data_provider::agent::ActiveModel;
use inspirer_data_provider::model::content::ContentBase;
use inspirer_data_provider::model::user::BeJoinedUserBase;
use inspirer_data_provider::result::PaginateWrapper;
use crate::result::{InspirerResp, ResponseMessage};

#[get("/content")]
pub fn get_contents(params: web::Query<GetContentsIndex>, state: web::Data<State>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let pool = state.db_conn.read().clone();
    Box::new(web::block(move || {
        params.activate(&pool.get().unwrap())
    })
        .from_err()
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
                list: data, last_page: r.last_page, total: r.total
            })))
        }))
}