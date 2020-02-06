use actix_redis::{Command, RedisActor};
use actix_service::ServiceFactory;
use actix_web::{App, HttpResponse, HttpServer, web};
use jsonwebtoken::{Algorithm, encode, EncodingKey, Header};
use redis_async::resp_array;

use inspirer_actix::middleware::auth::JwtToken;
use inspirer_application::app::{Config, State};
use inspirer_application::middleware::auth::Credential;
use inspirer_application::routes::scoped_admin;
use inspirer_data_provider::db::ConnPoolManager;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    start_server().await
}

async fn start_server() -> std::io::Result<()> {
    let config = Config::default();
    let db_conn = ConnPoolManager::builder().writer(config.db.writer.clone()).build();
    let redis_actor = RedisActor::start(config.redis_url.as_str());

    if let Some(password) = config.redis_password.as_ref() {
        redis_actor.send(Command(resp_array!["AUTH", password])).await.unwrap();
    }

    HttpServer::new(move || {
        App::new()
            .data(redis_actor.clone())
            .data(State {
                db_conn: db_conn.clone()
            })
            .service(web::scope("/api/admin")
                .wrap(JwtToken::<Credential>::new("secret"))
                .configure(scoped_admin)
            )
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