extern crate core;

use env_logger::Target;
use log::{info, LevelFilter};
use rocket;
use rocket::routes;

use crate::config::AppConfig;
use crate::idgen::IdGenerator;

mod http;
mod error;
mod idgen;
mod config;

#[rocket::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .target(Target::Stdout)
        .init();

    let config = AppConfig::new().unwrap();
    let id_generator = IdGenerator::create(&config.idgen);

    info!("Starting idgen-rs");
    let _ = rocket::build()
        .mount("/", routes![http::generate_ids, http::parse_id])
        .manage(config)
        .manage(id_generator)
        .launch().await;
}
