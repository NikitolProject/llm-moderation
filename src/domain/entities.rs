use chrono::{DateTime, Utc};
use models::DangerCategory;

use super::value_objects::{DangerScore, MessageId};

#[derive(Debug, Clone)]
pub struct ModerationResult {
    pub id: MessageId,
    pub message: String,
    pub danger_score: DangerScore,
    pub categories: Vec<DangerCategory>,
    pub created_at: DateTime<Utc>,
}

impl ModerationResult {
    pub fn new(
        message: String,
        danger_score: DangerScore,
        categories: Vec<DangerCategory>,
    ) -> Self {
        Self {
            id: MessageId::new(),
            message,
            danger_score,
            categories,
            created_at: Utc::now(),
        }
    }

    pub fn requires_review(&self, threshold: f32) -> bool {
        self.danger_score.requires_review(threshold)
    }
}

#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub danger_score: f32,
    pub categories: Vec<DangerCategory>,
}
