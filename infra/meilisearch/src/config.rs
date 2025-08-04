use serde::Deserialize;
use once_cell::sync::Lazy;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub connection_addr: String,
    pub master_key: String,
    pub video_index_name: String,
    pub timestamp_index_name: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let mut config = Config {
        connection_addr: "http://localhost:7700".to_string(),
        master_key: "masterkey28562856".to_string(),
        video_index_name: "video_index".to_string(),
        timestamp_index_name: "timestamp_index".to_string(),
    };

    if let Ok(env_config) = envy::prefixed("MEILI_").from_env::<Config>() {
        config = env_config;
    }

    tracing::info!("MeiliSearch configuration: {:?}", config);
    config
});