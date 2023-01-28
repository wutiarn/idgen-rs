use std::fmt::format;
use std::sync::Mutex;
use log::info;

const TIMESTAMP_BITS: usize = 35;
const COUNTER_BITS: usize = 35;
const INSTANCE_ID_BITS: usize = 35;
const DOMAIN_BITS: usize = 35;
const RESERVED_SECONDS_COUNT: usize = 35;

const DOMAINS_COUNT: usize = usize::pow(2, DOMAIN_BITS as u32);

struct IdGenerator {
    domain_state_holders: [Mutex<DomainStateHolder>; DOMAINS_COUNT],
}

struct DomainStateHolder {
    timestamp: u64,
    counter: u64,
}

impl IdGenerator {
    pub fn generate_id(&self) {
        let domain: usize = 0;
        let mutex = self.domain_state_holders.get(0)
            .expect(&format!("domain_state_holders should contain state for domain {domain}"));
        let state = mutex.lock().unwrap();
        info!("Successfully acquired state")
    }
}
