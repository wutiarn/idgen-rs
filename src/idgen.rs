use std::fmt::format;
use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use chrono::{DateTime, NaiveDate, Utc};
use log::info;

struct IdGeneratorConfig {
    timestamp_bits: u8,
    counter_bits: u8,
    instance_id_bits: u8,
    domain_id_bits: u8,
    epoch_start_second: u64,
    reserved_seconds_count: u64,
}

impl IdGeneratorConfig {
    pub fn validate(&self) {
        let bits_count: u32 = 0u32
            + self.timestamp_bits as u32
            + self.counter_bits as u32
            + self.instance_id_bits as u32
            + self.domain_id_bits as u32;
        assert!(bits_count <= 63, "bits sum must be less or equal to 63");
        assert!(get_current_timestamp(&self) > 0, "epoch_start_second must be in the past");
    }

    pub fn get_domains_count(&self) -> u64 {
        return 2u64.pow(self.domain_id_bits as u32) - 1;
    }

    pub fn get_max_counter_value(&self) -> u64 {
        return 2u64.pow(self.counter_bits as u32) - 1;
    }
}

struct IdGenerator {
    config: Arc<IdGeneratorConfig>,
    domain_state_holders: Vec<Mutex<DomainStateHolder>>,
}

impl IdGenerator {
    pub fn create(config: IdGeneratorConfig) -> IdGenerator {
        config.validate();
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

    pub fn generate_id(&self, domain: usize) {
        let mutex = self.domain_state_holders.get(domain)
            .expect(&format!("domain_state_holders should contain state for domain {domain}"));
        let state = mutex.lock().unwrap();
        state.generate_ids()
    }
}

struct DomainStateHolder {
    config: Arc<IdGeneratorConfig>,
    domain: u64,
    timestamp: u64,
    counter: u64,
}

impl DomainStateHolder {
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

    fn increment_counter(&mut self) {
        let config = &*self.config;
        let now = get_current_timestamp(&config);
        let time_delta = now - self.timestamp;

        if time_delta > config.reserved_seconds_count {
            self.timestamp = now - config.reserved_seconds_count;
            self.counter = 0;
            return;
        }

        if self.counter < config.get_max_counter_value() {
            self.counter += 1;
            return;
        }

        if time_delta > 0 {
            self.timestamp = self.timestamp + 1;
            self.counter = 0;
            return;
        }

        let now_millis = Utc::now().timestamp_millis() as u64;
        let sleep_millis = (now + 1) * 1000 - now_millis + 1;
        thread::sleep(Duration::from_millis(sleep_millis));
        self.timestamp = now + 1;
        self.counter = 0;
    }
}

fn get_current_timestamp(config: &IdGeneratorConfig) -> u64 {
    let current_unix_timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    return current_unix_timestamp - config.epoch_start_second;
}

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
            epoch_start_second: 1672531200,
            reserved_seconds_count: 0,
        };
        let generator = IdGenerator::create(config);
        generator.generate_id(0);
    }
}
