use actix_web::*;

use state::AppState;
use util::message::PaginatedListMessage;

#[macro_export]
macro_rules! paginator {
    ($conn:ident, $w:ident, $rt:ty, $lg:block) => {
        {
            let paginator = || -> Result<PaginatedListMessage<$rt>, Error> {
                let counter = || { $lg };
                let getter = || { $lg };

                let count = counter().count().first::<i64>($conn).map_err(error::ErrorInternalServerError)?;
                let results = getter()
                    .limit($w.per_page)
                    .offset(($w.page - 1) * $w.per_page)
                    .load::<$rt>($conn).map_err(error::ErrorInternalServerError)?;

                Ok(PaginatedListMessage { list: results, total: count, page: $w.page, per_page: $w.per_page })
            };

            paginator
        }
    };
}

pub fn get_paginate_params(req: &HttpRequest<AppState>) -> (Option<i64>, Option<i64>) {
    let query = req.query();

    let page = parse_number(query.get("page"));
    let per_page = parse_number(query.get("per_page"));

    (page, per_page)
}

fn parse_number<'a>(origin: Option<&'a str>) -> Option<i64> {
    origin.map(|r| {
        match r.to_string().parse::<i64>().ok() {
            Some(x) => {
                if x < 1 {
                    1
                } else {
                    x
                }
            },
            None => None,
        }
    })
}