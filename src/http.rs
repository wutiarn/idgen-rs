use std::collections::HashMap;
use rocket;
use rocket::serde::json::Json;
use rocket::State;
use crate::error::HttpError;
use crate::idgen::IdGenerator;
use serde::Serialize;
use crate::config::AppConfig;

#[rocket::get("/generate?<count>&<domains>")]
pub fn generate_ids(count: Option<u32>, domains: Option<&str>, id_generator: &State<IdGenerator>) -> Result<Json<GenerateIdsResponse>, HttpError> {
    let count = match count {
        Some(c) => c,
        None => 10
    };
    if count <= 0 {
       return Err(HttpError::BadRequest("count must be greater than 0".into()))
    }

    let domains: Vec<u64> = match domains {
        Some(domains_str) => {
            let domain_strs = domains_str.split(",");
            let mut result = Vec::with_capacity((id_generator.get_max_domain() + 1) as usize);
            for s in domain_strs {
                let parse_result = s.parse::<u64>();
                match parse_result {
                    Ok(i) => result.push(i),
                    Err(e) => return Err(HttpError::BadRequest(format!("Failed to parse domain '{s}' to u64")))
                }
            }
            result
        },
        None => (0..=id_generator.get_max_domain()).collect()
    };

    let mut ids_by_domain = HashMap::with_capacity(domains.len());
    for domain in domains {
        let ids = id_generator.generate_ids(count as usize, domain as usize);
        ids_by_domain.insert(domain, ids);
    }
    Ok(Json(GenerateIdsResponse { ids_by_domain }))
}

#[derive(Serialize, Debug)]
pub struct GenerateIdsResponse {
    pub ids_by_domain: HashMap<u64, Vec<u64>>
}
