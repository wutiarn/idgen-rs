use confique::Config;

#[derive(Config)]
pub struct AppConfig {
    #[config(nested)]
    pub idgen: IdGenConfig,
}

#[derive(Config)]
pub struct IdGenConfig {
    #[config(env = "INSTANCE_ID")]
    pub instance_id: u64,
    #[config(default = 35)]
    pub timestamp_bits: u8,
    #[config(default = 14)]
    pub counter_bits: u8,
    #[config(default = 6)]
    pub instance_id_bits: u8,
    #[config(default = 8)]
    pub domain_id_bits: u8,
    #[config(default = 1672531200)]
    pub epoch_start_second: u64,
    #[config(default = 60)]
    pub reserved_seconds_count: u64,
}

impl AppConfig {
    pub fn new() -> Result<Self, confique::Error> {
        AppConfig::builder()
            .env()
            .file("config.yaml")
            .load()
    }
}
