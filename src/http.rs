use rocket;
use crate::error::HttpError;

#[rocket::get("/generate?<count>")]
pub fn generate_ids(count: Option<i32>) -> Result<String, HttpError> {
    let count = match count {
        Some(c) => c,
        None => 10
    };
    if count <= 0 {
       return Err(HttpError::BadRequest("count must be greater than 0".into()))
    }
    Ok(format!("OK: {count} ids"))
}
