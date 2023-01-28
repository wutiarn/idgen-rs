use std::collections::HashSet;
use chrono::{TimeZone, Utc};
use rocket;
use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;

use crate::error::HttpError;
use crate::idgen::{IdGenerationError, IdGenerator};

#[rocket::get("/generate?<count>&<domains>")]
pub fn generate_ids(count: Option<u32>, domains: Option<&str>, id_generator: &State<IdGenerator>) -> Result<Json<GenerateIdsResponse>, HttpError> {
    let count = match count {
        Some(c) => c,
        None => 10
    };
    if count <= 0 {
        return Err(HttpError::BadRequest("count must be greater than 0".into()));
    }

    let domains: Vec<u64> = match domains {
        Some(domains_str) => {
            let domain_strs = domains_str.split(",");
            let mut domain_set = HashSet::with_capacity((id_generator.get_max_domain() + 1) as usize);
            for s in domain_strs {
                let parse_result = s.parse::<u64>();
                match parse_result {
                    Ok(i) => {
                        domain_set.insert(i);
                    }
                    Err(_) => return Err(HttpError::BadRequest(format!("failed to parse domain '{s}' to u64")))
                }
            }
            domain_set.into_iter().collect()
        }
        None => (0..=id_generator.get_max_domain()).collect()
    };

    let mut ids_by_domain = Vec::with_capacity(domains.len());
    for domain in domains {
        let ids = match id_generator.generate_ids(count as usize, domain as usize) {
            Ok(ids) => ids,
            Err(e) => match e {
                IdGenerationError::IncorrectDomain(_) => return Err(HttpError::BadRequest(e.to_string()))
            }
        };
        ids_by_domain.push(IdsForDomain { domain, ids });
    }
    Ok(Json(GenerateIdsResponse { ids_by_domain }))
}

#[rocket::get("/parse?<id>")]
pub fn parse_id(id: u64, id_generator: &State<IdGenerator>) -> Json<ParseIdResponse> {
    let id_params = id_generator.decode(id);
    let epoch_start = id_generator.get_epoch_start();
    let timestamp = Utc.timestamp_opt((epoch_start + id_params.timestamp) as i64, 0).unwrap();
    return Json(
        ParseIdResponse {
            timestamp: id_params.timestamp,
            decoded_timestamp: timestamp.to_rfc3339(),
            counter: id_params.counter,
            instance_id: id_params.instance_id,
            domain: id_params.domain,
        }
    );
}

#[derive(Serialize, Debug)]
pub struct GenerateIdsResponse {
    pub ids_by_domain: Vec<IdsForDomain>,
}

#[derive(Serialize, Debug)]
pub struct IdsForDomain {
    pub domain: u64,
    pub ids: Vec<u64>,
}

#[derive(Serialize, Debug)]
pub struct ParseIdResponse {
    domain: u64,
    timestamp: u64,
    decoded_timestamp: String,
    instance_id: u64,
    counter: u64,
}
