use std::fmt::format;
use std::sync::{Arc, Mutex};
use std::time::{Instant, SystemTime};
use log::info;

struct IdGeneratorConfig {
    timestamp_bits: u8,
    counter_bits: u8,
    instance_id_bits: u8,
    domain_id_bits: u8,
    epoch_start: Instant,
    reserved_seconds_count: u32,
}

impl IdGeneratorConfig {
    pub fn validate(&self) {
        let bits_count: u32 = 0u32
            + self.timestamp_bits as u32
            + self.counter_bits as u32
            + self.instance_id_bits as u32
            + self.domain_id_bits as u32;
        assert!(bits_count <= 63, "bits sum must be less or equal to 63")
    }

    pub fn get_domains_count(&self) -> u64 {
        return 2u64.pow(self.domain_id_bits as u32) - 1;
    }
}

struct IdGenerator {
    config: Arc<IdGeneratorConfig>,
    domain_state_holders: Vec<Mutex<DomainStateHolder>>,
}

struct DomainStateHolder {
    config: Arc<IdGeneratorConfig>,
    domain: u64,
    timestamp: u64,
    counter: u64
}


impl  IdGenerator {
    pub fn create(config: IdGeneratorConfig) -> IdGenerator {
        let mut holders = Vec::new();
        let domains_count = config.get_domains_count();
        let config_rc = Arc::new(config);
        for i in 0..domains_count {
            holders.push(DomainStateHolder::new(i, Arc::clone(&config_rc)))
        }
        return IdGenerator {
            config: config_rc,
            domain_state_holders: holders,
        };
    }

    pub fn generate_id(&self) {
        let domain: usize = 0;
        let mutex = self.domain_state_holders.get(0)
            .expect(&format!("domain_state_holders should contain state for domain {domain}"));
        let state = mutex.lock().unwrap();
        state.generate_ids()
    }

}

impl  DomainStateHolder {
    pub fn new(domain: u64, config: Arc<IdGeneratorConfig>) -> Mutex<DomainStateHolder> {
        let holder = DomainStateHolder {
            config,
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

fn get_current_timestamp() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_id() {
        let config = IdGeneratorConfig {
            timestamp_bits: 35,
            counter_bits: 14,
            instance_id_bits: 6,
            domain_id_bits: 8,
            epoch_start: Instant::now(),
            reserved_seconds_count: 0,
        };
        let generator = IdGenerator::create(config);
        generator.generate_id();
    }
}
