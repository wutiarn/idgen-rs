mod http;
mod error;

use env_logger::Target;
use log::{info, LevelFilter};
use rocket;
use rocket::routes;

#[rocket::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .target(Target::Stdout)
        .init();
    info!("Starting idgen-rs");
    let _ = rocket::build()
        .mount("/", routes![http::generate_ids])
        .launch().await;
}
