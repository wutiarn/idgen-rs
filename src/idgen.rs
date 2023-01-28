use std::fmt::format;
use std::sync::Mutex;
use std::time::{Instant, SystemTime};
use arr_macro::arr;
use log::info;
use once_cell::sync::Lazy;

const TIMESTAMP_BITS: usize = 35;
const COUNTER_BITS: usize = 14;
const INSTANCE_ID_BITS: usize = 6;
const DOMAIN_BITS: usize = 8;

const DOMAINS_COUNT: usize = usize::pow(2, DOMAIN_BITS as u32);

static EPOCH_START: Lazy<Instant> = Lazy::new(|| Instant::now());
const RESERVED_SECONDS_COUNT: usize = 10;


struct IdGenerator {
    domain_state_holders: [Mutex<DomainStateHolder>; DOMAINS_COUNT],
}

struct DomainStateHolder {
    domain: u8,
    timestamp: u64,
    counter: u64,
}

impl IdGenerator {
    pub fn generate_id(&self) {
        let domain: usize = 0;
        let mutex = self.domain_state_holders.get(0)
            .expect(&format!("domain_state_holders should contain state for domain {domain}"));
        let state = mutex.lock().unwrap();
        state.generate_ids()
    }

    pub fn create() -> IdGenerator {
        let mut i = 0u8;
        let holders: [Mutex<DomainStateHolder>; DOMAINS_COUNT] = arr![DomainStateHolder::new({i}); 256];
        return IdGenerator {
            domain_state_holders: holders,
        };
    }
}

impl DomainStateHolder {
    pub fn new(domain: u8) -> Mutex<DomainStateHolder> {
        let holder = DomainStateHolder {
            domain,
            counter: 0,
            timestamp: 0,
        };
        Mutex::new(holder)
    }
    pub fn generate_ids(&self) {
        info!("Generating ids...")
    }
}

fn get_current_timestamp() {

}

#[cfg(test)]
mod tests {
    use super::*;
}
