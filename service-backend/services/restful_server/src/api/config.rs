
#[derive(serde::Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}


pub static SERVER_CONFIG: once_cell::sync::Lazy<ServerConfig> = once_cell::sync::Lazy::new(|| {
    envy::prefixed("TSS_SERVER_").from_env().unwrap()
});