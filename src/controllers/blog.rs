use std::rc::Rc;
use std::sync::Arc;
use actix_web::{HttpRequest, HttpResponse, Responder, AsyncResponder, error::Error as ActixError, fs::NamedFile};
use state::AppState;
use futures::{Future, future::err as FutErr, future::result as FutResult};
use tera::{Tera, Context};
use template::{TEMPLATES, get_global_context};
use models::content::{self, FindFilter};
use message::{PaginatedListMessage, Pagination};
use error::error_handler;
use chrono::NaiveDateTime;
use error::Error;
use std::result::Result as StdResult;
use regex;
use comrak::{markdown_to_html, ComrakOptions};
use result::Result;

#[derive(Serialize)]
struct Content {
    id: u32,
    name: Option<String>,
    title: String,
    description: String,
    published_at_o: Option<NaiveDateTime>,
    published_at: Option<String>,
}

pub fn home(req: HttpRequest<AppState>) -> impl Responder {
    let ref_req = Rc::new(req);
    let req_for_contents = Rc::clone(&ref_req);
    let req_for_err = Rc::clone(&ref_req);
    let default_content = Pagination {
        page: 1,
        per_page: 10,
        filter: Some(content::GetContents::default()),
    };

    req_for_contents.state().database.send(default_content).from_err()
        .and_then(|contents| {
            let contents: PaginatedListMessage<content::ContentDisplay> = contents?;
            let mut list: Vec<Content> = vec![];

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

            let mut context = Context::new();
            context.add("contents", &list);
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

fn content_list(req: Rc<HttpRequest<AppState>>, filter: Pagination<content::GetContents>) -> impl Responder {
    let req_for_contents = Rc::clone(&req);
    let req_for_err = Rc::clone(&req);

    req_for_contents.state().database.send(filter).from_err()
        .and_then(|contents| {
            let contents: PaginatedListMessage<content::ContentDisplay> = contents?;
            let mut list: Vec<Content> = vec![];

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

            let pagination = PaginatedListMessage {
                list,
                page: contents.page,
                per_page: contents.per_page,
                total: contents.total,
            };

            let mut context = Context::new();
            let pages = (pagination.total as f64 / pagination.per_page as f64).ceil() as i64;
            context.add("contents", &pagination.list);
            context.add("pages", &pages);
            context.add("current", &pagination.page);
            context.extend(get_global_context());

            let rendered = match TEMPLATES.render("list.html", &context) {
                Ok(r) => r,
                Err(e) => {
                    debug!("Error to render: list.html, error detail: {:?}", e);
                    "Render error".into()
                }
            };
            Ok(HttpResponse::Ok().body(rendered))
        })
        .map_err(error_handler(req_for_err))
        .responder()
}

pub fn article_list(req: HttpRequest<AppState>) -> impl Responder {
    let ref_req = Rc::new(req);
    let mut default_content = Pagination::<content::GetContents>::from_request(Rc::clone(&ref_req));
    let mut default_getter = content::GetContents::default();

    default_getter.category = Some(content::CategoryMatcher::NotZero);
    default_getter.content_type = Some(1);
    default_content.filter = Some(default_getter);

    content_list(ref_req, default_content)
}

fn find_content(req: Rc<HttpRequest<AppState>>, filter: Option<FindFilter>) -> StdResult<impl Future<Item=Result<content::ContentFullDisplay>, Error=Error>, ActixError> {
    let req_for_contents = Rc::clone(&req);
    let req_for_err = Rc::clone(&req);
    let name = match req_for_contents.match_info().get("name") {
        Some(name) => {
            let numeric = regex::Regex::new(r"^\d+$").unwrap();
            let name_string = regex::Regex::new(r"^\w+(-\w+)*$").unwrap();

            if numeric.is_match(name) {
                let id = name.parse::<u32>().unwrap();
                Ok(content::Find::ById(id))
            } else if name_string.is_match(name) {
                Ok(content::Find::ByName(name.into()))
            } else {
                Err(error_handler(req_for_err)(Error::bad_request_error(Some("[param]"), None)))
            }
        }
        None => Err(error_handler(req_for_err)(Error::bad_request_error(Some("[param]"), None))),
    };

    name.map(move |name| {
        req_for_contents.state().database
            .send(content::FindContent {
                inner: name,
                filter,
            })
            .from_err()
    })
}

pub fn content(req: HttpRequest<AppState>) -> impl Responder {
    let ref_req = Rc::new(req);
    let req_for_err = Rc::clone(&ref_req);

    match find_content(ref_req, None) {
        Ok(fut) => {
            fut
                .and_then(|res| {
                    #[derive(Serialize)]
                    struct Content {
                        title: String,
                        content: String,
                        published_at_o: Option<NaiveDateTime>,
                        published_at: Option<String>,
                    }

                    let origin: content::ContentFullDisplay = res?;
                    let data = Content {
                        content: markdown_to_html(&origin.content.unwrap_or(String::from("")), &ComrakOptions::default()),
                        title: origin.title,
                        published_at_o: origin.published_at,
                        published_at: origin.published_at.map(|v| v.format("%Y-%m-%d").to_string()),
                    };

                    let mut context = Context::new();
                    context.add("content", &data);
                    context.add("__sub_title", &data.title);
                    context.extend(get_global_context());
                    let rendered = match TEMPLATES.render("content.html", &context) {
                        Ok(r) => r,
                        Err(e) => "Render error".into()
                    };
                    Ok(HttpResponse::Ok().body(rendered))
                })
                .map_err(error_handler(req_for_err))
                .responder()
        }
        Err(err) => FutErr(err).responder()
    }
}

pub fn source(req: HttpRequest<AppState>) -> impl Responder {
    let ref_req = Rc::new(req);
    let req_for_contents = Rc::clone(&ref_req);
    let req_for_err = Rc::clone(&ref_req);

    match req_for_contents.match_info().get("name") {
        Some(name) => {
            if req_for_contents.state().static_assets_handle {
                if name == "favicon.ico" {
                    use std::io::ErrorKind;
                    use std::path::PathBuf;

                    let req_for_static = Rc::clone(&ref_req);
                    let req_for_error = Rc::clone(&ref_req);
                    let directory = Arc::clone(&req_for_static.state().public_path);
                    let path = match *directory {
                        Some(ref ref_directory) => PathBuf::from(ref_directory).join(name),
                        None => panic!("Error: Fatal system logic error"),
                    };

                    let result = NamedFile::open(path).map_err(move |err| {
                        match err.kind() {
                            ErrorKind::NotFound => error_handler(req_for_error)(Error::not_found_error(Some(err), None)),
                            _ => error_handler(req_for_error)(Error::bad_request_error(Some("[param]"), None))
                        }
                    });

                    let r = match result {
                        Ok(named) => {
                            match named.respond_to(&*req_for_contents) {
                                Ok(resp) => resp.respond_to(&*req_for_contents),
                                Err(_) => Err(error_handler(req_for_err)(Error::internal_server_error(Some("[param]"), None)))
                            }
                        },
                        Err(err) => Err(err),
                    };

                    return match r {
                        Ok(async_result) => async_result.responder(),
                        Err(err) => FutErr(err).responder()
                    };
                }
            }

            let r = find_content(ref_req, Some(FindFilter {
                as_page: Some(true)
            }));

            match r {
                Ok(fut) => {
                    fut
                        .and_then(|res| {
                            #[derive(Serialize)]
                            struct Content {
                                title: String,
                                content: String,
                            }

                            let origin: content::ContentFullDisplay = res?;
                            let data = Content {
                                content: markdown_to_html(&origin.content.unwrap_or(String::from("")), &ComrakOptions::default()),
                                title: origin.title,
                            };

                            let mut context = Context::new();
                            context.add("content", &data);
                            context.add("__sub_title", &data.title);
                            context.extend(get_global_context());
                            let rendered = match TEMPLATES.render("page.html", &context) {
                                Ok(r) => r,
                                Err(e) => "Render error".into()
                            };
                            Ok(HttpResponse::Ok().body(rendered))
                        })
                        .map_err(error_handler(req_for_err))
                        .responder()
                },
                Err(err) => FutErr(err).responder()
            }
        }
        None => FutErr(error_handler(req_for_err)(Error::bad_request_error(Some("[param]"), None))).responder(),
    }
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