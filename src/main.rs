extern crate core;

use std::sync::Arc;
use actix_web::{App, HttpServer};
use env_logger::Target;
use log::{info, LevelFilter};

use crate::config::AppConfig;
use crate::idgen::IdGenerator;

mod http;
mod error;
mod idgen;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .target(Target::Stdout)
        .init();

    let config = AppConfig::new().unwrap();
    let id_generator = actix_web::web::Data::new(IdGenerator::create(&config.idgen));

    info!("Starting idgen-rs");
    HttpServer::new(move || {
        App::new()
            .app_data(id_generator.clone())
            .service(http::generate_ids)
            .service(http::parse_id)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
