use std::sync::{Arc, RwLock};
use crate::service::TimeStampSearchService;
use leaky_bucket::RateLimiter;

#[derive(Clone)]
pub struct AppState {
    pub timestamp_search: Box<TimeStampSearchService>,
    pub limiter: Arc<RwLock<RateLimiter>>,
}

impl AppState{
    pub fn new(timestamp_search: TimeStampSearchService,
           limiter : RateLimiter)->Self {
        Self {
            timestamp_search: Box::new(timestamp_search),
            limiter: Arc::new(RwLock::new(limiter)),
        }
    }
}