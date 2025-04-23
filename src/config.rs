use serde::Deserialize;
use config::{Config, File, Environment};

#[derive(Debug, Deserialize, Clone)]
pub struct ThroneConfig {
    pub host: String,
    pub debug: bool,
}

impl ThroneConfig {
    pub fn from_env() -> Self {
        let builder = Config::builder()
            .add_source(File::with_name("config.json"))
            .add_source(Environment::with_prefix("APP").separator("__"));

        builder.build()
            .expect("Failed to build configuration")
            .try_deserialize::<ThroneConfig>()
            .expect("Failed to deserialize configuration")
    }
}