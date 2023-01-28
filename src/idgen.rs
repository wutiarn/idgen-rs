use std::fmt::format;
use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use chrono::{DateTime, NaiveDate, Utc};
use log::info;

pub struct IdGeneratorConfig {
    timestamp_bits: u8,
    counter_bits: u8,
    instance_id_bits: u8,
    domain_id_bits: u8,
    epoch_start_second: u64,
    reserved_seconds_count: u64,
}

struct IdGeneratorExtendedConfig {
    timestamp_bits: u8,
    counter_bits: u8,
    instance_id_bits: u8,
    domain_id_bits: u8,
    epoch_start_second: u64,
    reserved_seconds_count: u64,

    max_domain: usize,
    max_counter_value: u64,
    max_instance_id: u64,
}

impl IdGeneratorExtendedConfig {
    fn new(config: IdGeneratorConfig) -> IdGeneratorExtendedConfig {
        let result = IdGeneratorExtendedConfig {
            timestamp_bits: config.timestamp_bits,
            counter_bits: config.counter_bits,
            instance_id_bits: config.instance_id_bits,
            domain_id_bits: config.domain_id_bits,
            epoch_start_second: config.epoch_start_second,
            reserved_seconds_count: config.reserved_seconds_count,
            max_domain: IdGeneratorExtendedConfig::calculate_max_value_for_bits(config.domain_id_bits) as usize,
            max_counter_value: IdGeneratorExtendedConfig::calculate_max_value_for_bits(config.counter_bits),
            max_instance_id: IdGeneratorExtendedConfig::calculate_max_value_for_bits(config.instance_id_bits),
        };
        result.validate();
        result
    }

    fn validate(&self) {
        let bits_count: u32 = 0u32
            + self.timestamp_bits as u32
            + self.counter_bits as u32
            + self.instance_id_bits as u32
            + self.domain_id_bits as u32;
        assert!(bits_count <= 63, "bits sum must be less or equal to 63");
        assert!(get_current_timestamp(&self) > 0, "epoch_start_second must be in the past");
    }

    fn calculate_max_value_for_bits(bits_count: u8) -> u64 {
        return 2u64.pow(bits_count as u32) - 1;
    }
}

struct IdGenerator {
    config: Arc<IdGeneratorExtendedConfig>,
    domain_state_holders: Vec<Mutex<DomainStateHolder>>,
}

impl IdGenerator {
    pub fn create(config: IdGeneratorConfig) -> IdGenerator {
        let config = IdGeneratorExtendedConfig::new(config);
        let mut holders = Vec::new();
        let max_domain = config.max_domain;
        let config_rc = Arc::new(config);
        for i in 0..max_domain {
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
    config: Arc<IdGeneratorExtendedConfig>,
    domain: usize,
    timestamp: u64,
    counter: u64,
}

impl DomainStateHolder {
    pub fn new(domain: usize, config: Arc<IdGeneratorExtendedConfig>) -> Mutex<DomainStateHolder> {
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
        let nowTimestamp = get_current_timestamp(&config);
        let time_delta = nowTimestamp - self.timestamp;

        if time_delta > config.reserved_seconds_count {
            self.timestamp = nowTimestamp - config.reserved_seconds_count;
            self.counter = 0;
            return;
        }

        if self.counter < config.max_counter_value {
            self.counter += 1;
            return;
        }

        if time_delta > 0 {
            self.timestamp = self.timestamp + 1;
            self.counter = 0;
            return;
        }

        DomainStateHolder::wait_for_next_second();
        self.timestamp = nowTimestamp + 1;
        self.counter = 0;
    }

    fn wait_for_next_second() {
        let now = Utc::now();
        let sleep_millis = ((now.timestamp() + 1) * 1000 - now.timestamp_millis() + 1) as u64;
        thread::sleep(Duration::from_millis(sleep_millis));
    }
}

fn get_current_timestamp(config: &IdGeneratorExtendedConfig) -> u64 {
    let current_unix_timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    return current_unix_timestamp - config.epoch_start_second;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_id() {
        let config = build_config();
        let generator = IdGenerator::create(config);
        generator.generate_id(0);
    }

    #[test]
    fn sleep_until_next_second() {
        let config = Arc::new(IdGeneratorExtendedConfig::new(build_config()));
        let start_timestamp = get_current_timestamp(&config);
        let mut holder = DomainStateHolder {
            config: Arc::clone(&config),
            domain: 0,
            timestamp: start_timestamp,
            counter: config.max_counter_value,
        };

        holder.increment_counter();

        let end_timestamp = get_current_timestamp(&config);
        assert!(end_timestamp > start_timestamp, "timestamp was not incremented");
    }

    fn build_config() -> IdGeneratorConfig {
        return IdGeneratorConfig {
            timestamp_bits: 35,
            counter_bits: 14,
            instance_id_bits: 6,
            domain_id_bits: 8,
            epoch_start_second: 1672531200,
            reserved_seconds_count: 0,
        };
    }
}
