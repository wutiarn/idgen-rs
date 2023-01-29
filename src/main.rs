extern crate core;

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
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .target(Target::Stdout)
        .init();

    info!("Starting idgen-rs");
    HttpServer::new(|| {
        let config = AppConfig::new().unwrap();
        let id_generator = IdGenerator::create(&config.idgen);
        App::new()
            .app_data(actix_web::web::Data::new(id_generator))
            .service(http::generate_ids)
            .service(http::parse_id)
    })
        .bind(("0.0.0.0", 8080))
        .unwrap()
        .run()
        .await
        .unwrap();
}
