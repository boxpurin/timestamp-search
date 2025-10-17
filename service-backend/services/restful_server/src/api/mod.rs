pub mod route;
pub (crate)mod request;
pub (crate)mod response;
pub (crate)mod handle;
pub mod app_state;
pub mod service;
pub mod middleware;
pub mod config;

pub async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}