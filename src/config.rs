pub struct AppConfig {
    pub idgen: IdGenConfig
}

pub struct IdGenConfig {
    instance_id: u64,
    timestamp_bits: u8,
    counter_bits: u8,
    instance_id_bits: u8,
    domain_id_bits: u8,
    epoch_start_second: u64,
    reserved_seconds_count: u64,
}

impl AppConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        let c = config::Config::builder()
            .add_source(config::File::with_name("app_config").required(false))
            .add_source(config::File::with_name("app_config_local").required(false))
            .add_source(config::Environment::with_prefix("APP"))
            .set_default()
            .build()?;
        c.try_deserialize()
    }
}
