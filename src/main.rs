extern crate core;

mod http;
mod error;
mod idgen;
mod config;

use env_logger::Target;
use log::{info, LevelFilter};
use rocket;
use rocket::routes;
use crate::config::AppConfig;
use crate::idgen::IdGenerator;

#[rocket::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .target(Target::Stdout)
        .init();

    let config = AppConfig::new().unwrap();
    let idGenerator = IdGenerator::create(&config.idgen);

    info!("Starting idgen-rs");
    let _ = rocket::build()
        .mount("/", routes![http::generate_ids])
        .manage(config)
        .manage(idGenerator)
        .launch().await;
}
