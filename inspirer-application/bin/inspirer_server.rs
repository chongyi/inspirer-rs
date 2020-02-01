use actix_web::{HttpServer, App, web, HttpResponse};
use actix_service::ServiceFactory;
use inspirer_application::app::{Config, State};
use inspirer_data_provider::db::ConnPoolManager;
use inspirer_application::routes::scoped_admin;
use inspirer_actix::middleware::auth::JwtTokenAuth;
use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};
use inspirer_application::auth::Credential;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    start_server().await
}

async fn start_server() -> std::io::Result<()> {
    let config = Config::default();
    let db_conn = ConnPoolManager::builder().writer(config.db.writer.clone()).build();

    HttpServer::new(move || {
        App::new()
            .data(State {
                db_conn: db_conn.clone()
            })
            .service(web::scope("/api/admin").wrap(JwtTokenAuth::<Credential>::new("secret")).configure(scoped_admin))
            .route("/", web::get().to(|| {
                HttpResponse::Ok().body(encode(
                    &Header::new(Algorithm::HS256),
                    &Credential {
                        uuid: "a76c-jjk0-ryey-140l".to_owned(),
                        exp: (chrono::Utc::now() + chrono::Duration::minutes(5)).timestamp() as usize,
                    },
                    &EncodingKey::from_secret("secret".as_ref()),
                ).unwrap())
            }))
    })
        .workers(config.worker_num.unwrap_or(num_cpus::get()))
        .bind(config.listen.as_str())?
        .run()
        .await
}