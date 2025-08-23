use std::sync::Arc;
use crate::service::TimeStampSearchService;

pub struct AppState{
    pub timestamp_search: Arc<TimeStampSearchService>
}