use serde::Deserialize;
use serde::Serialize;

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
