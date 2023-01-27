use rocket;

#[rocket::main]
async fn main() {
    let _ = rocket::build().launch().await;
}
