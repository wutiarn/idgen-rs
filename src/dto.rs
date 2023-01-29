use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use serde::Serialize;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GenerateIdsRequest {
    pub count: Option<u32>,
    pub domains: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ParseIdRequest {
    pub id: u64,
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
    pub domain: u64,
    pub timestamp: u64,
    pub decoded_timestamp: String,
    pub instance_id: u64,
    pub counter: u64,
}

pub trait JsonResponder: Responder {

}

impl Responder for GenerateIdsResponse {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<BoxBody> {
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&self).unwrap())
    }
}

