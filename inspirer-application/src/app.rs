use inspirer_data_provider::prelude::*;
use actix_web::{App, web};
use actix_web::dev::{ServiceRequest, ServiceResponse, MessageBody};

pub type ActixWebApp = App<
    impl ServiceFactory<
        Config = (),
        Request = ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Error = actix_web::Error,
    >,
    impl MessageBody,
>;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub listen: String,
    pub worker_num: Option<usize>,
    pub pid_file: Option<String>,
    pub db: ConnectionConfig
}

impl Default for Config {
    fn default() -> Self {
        Config {
            listen: "127.0.0.1:8088".into(),
            worker_num: Some(num_cpus::get()),
            pid_file: None,
            db: ConnectionConfig::default()
        }
    }
}

#[derive(Clone)]
pub struct State {
    pub db_conn: ConnPoolManager
}

pub fn create_app(db_conn: ConnPoolManager) -> ActixWebApp {
    use super::routes::scoped_admin;
    use actix_web::HttpResponse;
    use inspirer_data_provider::db::ConnPoolManager;

    App::new()
        .data(State {
            db_conn
        })
        .service(web::scope("/admin").configure(scoped_admin))
        .route("/", web::get().to(|| HttpResponse::Ok().body("hello, world")))
}