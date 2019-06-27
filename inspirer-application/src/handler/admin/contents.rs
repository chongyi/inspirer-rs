use actix_web::web::{self, get, post, put, delete};
use actix_web::error::{Error, BlockingError};
use futures::Future;
use actix_web::HttpResponse;
use inspirer_data_provider::agent::content::GetContentsIndex;
use crate::app::State;
use inspirer_data_provider::agent::ActiveModel;
use inspirer_data_provider::model::content::ContentBase;
use inspirer_data_provider::model::user::BeJoinedUserBase;
use inspirer_data_provider::result::PaginateWrapper;

#[get("/content")]
pub fn get_contents(params: web::Query<GetContentsIndex>, state: web::Data<State>) -> impl Future<Item=HttpResponse, Error=Error> {
    let pool = state.db_conn.read().clone();
    web::block(move || {
        params.activate(&pool.get().unwrap())
    })
        .from_err()
        .and_then(|r| {
            #[derive(Serialize)]
            struct Item<'i> {
                content: &'i ContentBase,
                creator: &'i BeJoinedUserBase,
            }

            let data = r.data.iter().map(|(content, creator)| {
                Item { content, creator }
            }).collect();

            Ok(HttpResponse::Ok().json(&PaginateWrapper {
                data, last_page: r.last_page, total: r.total
            }))
        })
}