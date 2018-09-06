#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate tera;
#[macro_use] extern crate log;

extern crate serde;
extern crate serde_json;
extern crate actix;
extern crate actix_web;
extern crate mime;
extern crate chrono;
extern crate futures;
extern crate regex;
extern crate comrak;
extern crate toml;
extern crate rss;
extern crate url;
extern crate tempdir;

#[macro_use] mod database;
mod models;
mod schema;
mod error;
mod message;
mod controllers;

mod template {
    use tera::Tera;
    use tera::Context;
    use std::fs::File;
    use std::env;
    use std::io::Read;
    use models::site_setting::SiteSetting;
    use toml;
    lazy_static! {
        pub static ref TEMPLATES: Tera = {
            let mut tera = compile_templates!("res/templates/**/*");
            tera.autoescape_on(vec!["html", ".sql"]);
            tera
        };
    }

    pub fn get_site_setting() -> SiteSetting {
        match env::var("SITE_SETTING_FILE") {
            Ok(file) => {
                let mut file = File::open(&file);
                match file {
                    Ok(ref mut result) => {
                        let mut contents = String::new();
                        let state = result.read_to_string(&mut contents).is_ok();

                        if state {
                            toml::from_str(&contents).unwrap_or(SiteSetting::default())
                        } else {
                            SiteSetting::default()
                        }
                    },
                    Err(_) => SiteSetting::default()
                }
            },
            Err(_) => SiteSetting::default()
        }
    }

    pub fn get_global_context() -> Context {
        let mut context = Context::new();
        let setting: SiteSetting = get_site_setting();

        context.add("__site_setting", &setting);
        context
    }
}

pub mod state;
pub mod routes;
pub mod utils;

pub mod result {
    use std::result::Result as StdResult;
    use error::Error;

    pub type Result<T> = StdResult<T, Error>;
}