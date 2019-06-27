use inspirer_data_provider::prelude::*;
use actix_web::App;

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