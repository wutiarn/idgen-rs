use rocket;

#[rocket::get("/generate")]
pub fn generate_ids() -> Result<String, String> {
    Ok("OK".into())
}
