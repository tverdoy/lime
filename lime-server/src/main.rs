mod service;
mod config;
mod utils;

use actix_web::{App, HttpServer};
use structured_logger::async_json::new_writer;
use structured_logger::Builder;
use crate::config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = match Config::load_from_file() {
        Ok(config) => config,
        Err(err) => {
            panic!("load config: {err}", err = err)
        }
    };
    init_logger(&config);

    HttpServer::new(|| {
        App::new()
            .service(service::route::get_route())
    })
        .bind(config.server().addr())?
        .run()
        .await
}

fn init_logger(_: &Config) {
    Builder::new()
        .with_target_writer("*", new_writer(tokio::io::stdout()))
        .init();
}