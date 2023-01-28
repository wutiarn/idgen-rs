use std::cmp::max;
use std::fmt::format;
use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::{result, thread};
use std::time::{Duration, Instant, SystemTime};
use chrono::{DateTime, NaiveDate, Utc};
use log::{debug, info, warn};
use crate::config::IdGenConfig;
use thiserror::Error;

pub struct IdGenerator {
    config: Arc<IdGeneratorExtendedConfig>,
    domain_state_holders: Vec<Mutex<DomainStateHolder>>,
}

impl IdGenerator {
    pub fn create(config: &IdGenConfig) -> IdGenerator {
        let config = IdGeneratorExtendedConfig::new(config);
        let start_timestamp = get_current_timestamp(&config);
        debug!("Initializing id generator with start timestamp {start_timestamp}");
        let mut holders = Vec::with_capacity((config.max_domain + 1) as usize);
        let max_domain = config.max_domain;
        let config_rc = Arc::new(config);
        for i in 0..=max_domain {
            holders.push(DomainStateHolder::new(i as u64, Arc::clone(&config_rc), start_timestamp))
        }
        return IdGenerator {
            config: config_rc,
            domain_state_holders: holders,
        };
    }

    pub fn generate_ids(&self, count: usize, domain: usize) -> Result<Vec<u64>, IdGenerationError> {
        let mutex = match self.domain_state_holders.get(domain) {
            Some(m) => m,
            None => return Err(IdGenerationError::IncorrectDomain(domain))
        };
        let mut lock_result = mutex.lock();
        let state = lock_result.as_mut().unwrap();
        Ok(state.generate_ids(count, domain))
    }

    pub fn get_max_domain(&self) -> u64 {
        return self.config.max_domain;
    }
}

#[derive(Error, Debug)]
pub enum IdGenerationError {
    #[error("incorrect domain: {0}")]
    IncorrectDomain(usize)
}

struct IdGeneratorExtendedConfig {
    instance_id: u64,
    timestamp_bits: u8,
    counter_bits: u8,
    instance_id_bits: u8,
    domain_id_bits: u8,
    epoch_start_second: u64,
    reserved_seconds_count: u64,

    max_timestamp: u64,
    max_instance_id: u64,
    max_counter_value: u64,
    max_domain: u64,
}

impl IdGeneratorExtendedConfig {
    fn new(config: &IdGenConfig) -> IdGeneratorExtendedConfig {
        let result = IdGeneratorExtendedConfig {
            instance_id: config.instance_id,
            timestamp_bits: config.timestamp_bits,
            counter_bits: config.counter_bits,
            instance_id_bits: config.instance_id_bits,
            domain_id_bits: config.domain_id_bits,
            epoch_start_second: config.epoch_start_second,
            reserved_seconds_count: config.reserved_seconds_count,
            max_timestamp: IdGeneratorExtendedConfig::calculate_max_value_for_bits(config.timestamp_bits),
            max_instance_id: IdGeneratorExtendedConfig::calculate_max_value_for_bits(config.instance_id_bits),
            max_counter_value: IdGeneratorExtendedConfig::calculate_max_value_for_bits(config.counter_bits),
            max_domain: IdGeneratorExtendedConfig::calculate_max_value_for_bits(config.domain_id_bits),
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
        let max_usize = usize::MAX;
        assert!(self.max_domain + 1 <= usize::MAX as u64, "max domain must not exceed usize");
        assert!(get_current_timestamp(&self) > 0, "epoch_start_second must be in the past");
    }

    fn calculate_max_value_for_bits(bits_count: u8) -> u64 {
        return 2u64.pow(bits_count as u32) - 1;
    }
}

struct DomainStateHolder {
    config: Arc<IdGeneratorExtendedConfig>,
    domain: u64,
    timestamp: u64,
    counter: u64,
}

impl DomainStateHolder {
    pub fn new(domain: u64, config: Arc<IdGeneratorExtendedConfig>, start_timestamp: u64) -> Mutex<DomainStateHolder> {
        let holder = DomainStateHolder {
            config,
            domain,
            counter: 0,
            timestamp: start_timestamp,
        };
        Mutex::new(holder)
    }

    pub fn generate_ids(&mut self, count: usize, domain: usize) -> Vec<u64> {
        let mut vec: Vec<u64> = Vec::with_capacity(count);
        let config = &*Arc::clone(&self.config);
        self.update_timestamp(config);
        for i in 0..count {
            self.increment_counter(config);
            let params = IdParams {
                timestamp: self.timestamp,
                counter: self.counter,
                instance_id: self.config.instance_id,
                domain: self.domain,
            };
            let encoded = params.encode(config);
            vec.push(encoded)
        }
        return vec;
    }

    fn update_timestamp(&mut self, config: &IdGeneratorExtendedConfig) {
        let now_timestamp = get_current_timestamp(&config);
        let time_delta = now_timestamp - self.timestamp;

        if time_delta > config.reserved_seconds_count {
            self.timestamp = now_timestamp - config.reserved_seconds_count;
            self.counter = 0;
            debug!("Using earliest reserve second: {0}. domain={1}", self.timestamp, self.domain);
            return;
        }

        if self.counter < config.max_counter_value {
            return;
        }

        if time_delta > 0 {
            self.timestamp = self.timestamp + 1;
            self.counter = 0;
            debug!("Using next reserve second: {0} ({1} left). domain={2}.", self.timestamp, time_delta - 1, self.domain);
            return;
        }

        self.wait_for_next_second();
        self.timestamp = now_timestamp + 1;
        self.counter = 0;
        debug!("Using realtime second: {0}. domain={1}", self.timestamp, self.domain);
    }

    fn increment_counter(&mut self, config: &IdGeneratorExtendedConfig) {
        if self.counter >= config.max_counter_value {
            self.update_timestamp(config);
        }
        self.counter += 1;
    }

    fn wait_for_next_second(&self) {
        let now = Utc::now();
        let sleep_millis = ((now.timestamp() + 1) * 1000 - now.timestamp_millis() + 1) as u64;
        warn!("Seconds reserve exhausted. Sleeping {0}ms. domain={1}", sleep_millis, self.domain);
        thread::sleep(Duration::from_millis(sleep_millis));
    }
}

#[derive(PartialEq, Debug)]
struct IdParams {
    timestamp: u64,
    counter: u64,
    instance_id: u64,
    domain: u64,
}

impl IdParams {
    fn encode(&self, config: &IdGeneratorExtendedConfig) -> u64 {
        let mut result = 0u64;
        result = IdParams::encode_part(result, self.timestamp, config.max_timestamp, config.timestamp_bits);
        result = IdParams::encode_part(result, self.instance_id, config.max_instance_id, config.instance_id_bits);
        result = IdParams::encode_part(result, self.counter, config.max_counter_value, config.counter_bits);
        result = IdParams::encode_part(result, self.domain, config.max_domain, config.domain_id_bits);
        return result;
    }

    fn decode(encoded: u64, config: &IdGeneratorExtendedConfig) -> IdParams {
        let mut src = encoded;
        let (domain, src) = IdParams::decode_part(src, config.max_domain, config.domain_id_bits);
        let (counter, src) = IdParams::decode_part(src, config.max_counter_value, config.counter_bits);
        let (instance_id, src) = IdParams::decode_part(src, config.max_instance_id, config.instance_id_bits);
        let (timestamp, src) = IdParams::decode_part(src, config.max_timestamp, config.timestamp_bits);
        IdParams {
            timestamp,
            counter,
            instance_id,
            domain,
        }
    }

    fn encode_part(target: u64, value: u64, mask: u64, bits: u8) -> u64 {
        let masked = value & mask;
        assert_eq!(masked, value, "value must not exceed max_value");
        return target << bits | masked;
    }

    fn decode_part(src: u64, mask: u64, bits: u8) -> (u64, u64) {
        let value = src & mask;
        let remainder = src >> bits;
        return (value, remainder);
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
        let generator = IdGenerator::create(&config);
        let generated = generator.generate_ids(100, 5).unwrap();
        assert_eq!(generated.len(), 100);
    }

    #[test]
    fn sleep_until_next_second() {
        let config = Arc::new(IdGeneratorExtendedConfig::new(&build_config()));
        let start_timestamp = get_current_timestamp(&config);
        let mut holder = DomainStateHolder {
            config: Arc::clone(&config),
            domain: 0,
            timestamp: start_timestamp,
            counter: config.max_counter_value,
        };

        holder.increment_counter(&config);

        let end_timestamp = get_current_timestamp(&config);
        assert!(end_timestamp > start_timestamp, "timestamp was not incremented");
    }

    fn build_config() -> IdGenConfig {
        return IdGenConfig {
            instance_id: 1,
            timestamp_bits: 35,
            counter_bits: 14,
            instance_id_bits: 6,
            domain_id_bits: 8,
            epoch_start_second: 1672531200,
            reserved_seconds_count: 0,
        };
    }

    #[test]
    fn encode_decode_id() {
        let config = IdGeneratorExtendedConfig::new(&build_config());
        let params = IdParams {
            timestamp: 1458569,
            counter: 1,
            instance_id: 5,
            domain: 9,
        };
        let encoded = params.encode(&config);
        assert_eq!(encoded, 391531655594249, "should generate expected id");

        let decoded = IdParams::decode(encoded, &config);
        assert_eq!(params, decoded, "decoded should be equals to initial params")
    }
}
