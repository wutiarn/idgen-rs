use std::collections::HashSet;
use chrono::{TimeZone, Utc};
use serde::Serialize;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::http::header::ContentType;
use actix_web::web::Query;

use crate::error::HttpError;
use crate::idgen::{IdGenerationError, IdGenerator};

#[actix_web::get("/generate")]
pub fn generate_ids(query: Query<GenerateIdsRequest>, id_generator: &web::Data<IdGenerator>) -> actix_web::Result<HttpResponse, HttpError> {

    let count = match query.count {
        Some(c) => c,
        None => 10
    };
    if count <= 0 {
        return Err(HttpError::BadRequest("count must be greater than 0".into()));
    }

    let domains: Vec<u64> = match query.domains {
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
    let response = HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(GenerateIdsResponse { ids_by_domain });
    OK(response)
}

#[actix_web::get("/parse?<id>")]
pub fn parse_id(id: u64, id_generator: &web::Data<IdGenerator>) -> HttpResponse {
    let id_params = id_generator.decode(id);
    let epoch_start = id_generator.get_epoch_start();
    let timestamp = Utc.timestamp_opt((epoch_start + id_params.timestamp) as i64, 0).unwrap();
    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(
            ParseIdResponse {
                timestamp: id_params.timestamp,
                decoded_timestamp: timestamp.to_rfc3339(),
                counter: id_params.counter,
                instance_id: id_params.instance_id,
                domain: id_params.domain,
            }
        );
}

pub struct GenerateIdsRequest<'t> {
    pub count: Option<u32>,
    pub domains: Option<&'t str>
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
