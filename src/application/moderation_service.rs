use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{DangerScore, LlmClient, ModerationResult, ResultStore};

use super::errors::ApplicationError;

pub struct ModerationService {
    llm_client: Arc<dyn LlmClient>,
    result_store: Arc<dyn ResultStore>,
    danger_threshold: f32,
}

impl ModerationService {
    pub fn new(
        llm_client: Arc<dyn LlmClient>,
        result_store: Arc<dyn ResultStore>,
        danger_threshold: f32,
    ) -> Self {
        Self {
            llm_client,
            result_store,
            danger_threshold,
        }
    }

    pub fn danger_threshold(&self) -> f32 {
        self.danger_threshold
    }

    pub async fn moderate(&self, message: String) -> Result<ModerationResult, ApplicationError> {
        let analysis = self.llm_client.analyze(&message).await?;

        let result = ModerationResult::new(
            message,
            DangerScore::new(analysis.danger_score),
            analysis.categories,
        );

        self.result_store.save(&result).await?;

        Ok(result)
    }

    pub async fn get_reason(&self, id: Uuid) -> Result<(ModerationResult, String), ApplicationError> {
        let result = self
            .result_store
            .get(id)
            .await?
            .ok_or_else(|| ApplicationError::NotFound(id.to_string()))?;

        let score = result.danger_score.value();
        if score <= self.danger_threshold {
            return Err(ApplicationError::ReasonNotAvailable {
                score,
                threshold: self.danger_threshold,
            });
        }

        let reason = self.llm_client.explain(&result.message, score).await?;

        Ok((result, reason))
    }

    pub async fn get_moderation(&self, id: Uuid) -> Result<Option<ModerationResult>, ApplicationError> {
        Ok(self.result_store.get(id).await?)
    }
}
