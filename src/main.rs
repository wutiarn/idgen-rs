use env_logger::Target;
use log::{info, LevelFilter};
use rocket;

#[rocket::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .target(Target::Stdout)
        .init();
    info!("Starting idgen-rs");
    let _ = rocket::build().launch().await;
}
