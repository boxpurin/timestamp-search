pub mod app_state;
pub mod config;
pub(crate) mod handle;
pub mod middleware;
pub(crate) mod request;
pub(crate) mod response;
pub mod route;
pub mod service;

#[allow(dead_code)]
pub async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
