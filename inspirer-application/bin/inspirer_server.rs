use actix_web::{HttpServer, App, web, HttpResponse};
use inspirer_application::app::{Config, State};
use inspirer_data_provider::db::ConnPoolManager;
use inspirer_application::routes::scoped_admin;

fn main() {
    start_server();
}

fn start_server() {
    let sys = actix_rt::System::new("inspirer");

    let config = Config::default();
    let db_conn = ConnPoolManager::builder().writer(config.db.writer.clone()).build();

    HttpServer::new(move || {
        App::new()
            .data(State {
                db_conn: db_conn.clone()
            })
            .service(web::scope("/admin").configure(scoped_admin))
            .route("/", web::get().to(|| HttpResponse::Ok().body("hello, world")))
    })
        .workers(config.worker_num.unwrap_or(num_cpus::get()))
        .bind(config.listen.as_str())
        .unwrap()
        .start();

    let _ = sys.run();
}