use inspirer_data_provider::prelude::*;
use actix_service::ServiceFactory;
use actix_web::{App, web};
use actix_web::dev::{ServiceRequest, ServiceResponse, MessageBody};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub listen: String,
    pub worker_num: Option<usize>,
    pub pid_file: Option<String>,
    pub db: ConnectionConfig,
    pub redis_url: String,
    pub redis_password: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            listen: "127.0.0.1:8088".into(),
            worker_num: Some(num_cpus::get()),
            pid_file: None,
            db: ConnectionConfig::default(),
            redis_url: "127.0.0.1:6379".into(),
            redis_password: None,
        }
    }
}

#[derive(Clone)]
pub struct State;

impl State {
    pub fn new() -> Self {
        // TODO 后续管理全局注册的状态、配置信息
        State
    }
}