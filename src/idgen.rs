use std::fmt::format;
use std::sync::Mutex;
use log::info;

const TIMESTAMP_BITS: usize = 35;
const COUNTER_BITS: usize = 14;
const INSTANCE_ID_BITS: usize = 6;
const DOMAIN_BITS: usize = 8;
const RESERVED_SECONDS_COUNT: usize = 10;

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
        state.generate_ids()
    }

    pub fn create() -> IdGenerator {
        let mut holders: [Mutex<DomainStateHolder>; DOMAINS_COUNT] = [];
        for i in 0..DOMAINS_COUNT {
            let holder = DomainStateHolder {
                counter: 0,
                timestamp: 0
            };
            holders[i] = Mutex::new(holder)
        }
        return IdGenerator {
            domain_state_holders: holders,
        }
    }
}

impl DomainStateHolder {
    pub fn generate_ids(&self) {
        info!("Generating ids...")
    }
}

#[cfg(test)]
mod tests {
    use super::*;


}
