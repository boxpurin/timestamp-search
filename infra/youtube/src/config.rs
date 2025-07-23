use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct YouTubeClient {
    pub google_client_secret_path: String,
    pub persistent_token_path: String,
}

pub static YOUTUBE_CLIENT: Lazy<YouTubeClient> = Lazy::new(|| {
    envy::prefixed("TSSEARCH_")
        .from_env::<YouTubeClient>()
        .unwrap()
});
