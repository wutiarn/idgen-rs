use rocket;
use rocket::serde::json::Json;
use rocket::State;
use crate::error::HttpError;
use crate::idgen::IdGenerator;
use serde::Serialize;

#[rocket::get("/generate?<count>")]
pub fn generate_ids(count: Option<i32>, id_generator: &State<IdGenerator>) -> Result<Json<GenerateIdsResponse>, HttpError> {
    let count = match count {
        Some(c) => c,
        None => 10
    };
    if count <= 0 {
       return Err(HttpError::BadRequest("count must be greater than 0".into()))
    }
    let ids = id_generator.generate_ids(count as usize, 0);
    Ok(Json(GenerateIdsResponse { ids }))
}

#[derive(Serialize, Debug)]
pub struct GenerateIdsResponse {
    pub ids: Vec<u64>
}
