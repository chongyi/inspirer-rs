use inspirer_actix_ext::service::{IntoService, DependencyFactory};

#[derive(Service, FromRequest)]
pub struct ContentService;

impl ContentService {
    pub fn echo(&self) -> &'static str {
        "hello world"
    }
}