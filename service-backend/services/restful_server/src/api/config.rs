#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

pub static SERVER_CONFIG: once_cell::sync::Lazy<ServerConfig> =
    once_cell::sync::Lazy::new(|| envy::prefixed("TSS_APP_").from_env().unwrap());

impl ServerConfig {
    pub fn listen_addr(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}
