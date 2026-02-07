use std::sync::Arc;

use crate::application::ModerationService;
use crate::infrastructure::AppConfig;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub moderation_service: Arc<ModerationService>,
}

impl AppState {
    pub fn new(config: AppConfig, moderation_service: ModerationService) -> Self {
        Self {
            config: Arc::new(config),
            moderation_service: Arc::new(moderation_service),
        }
    }
}
