use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

use super::entities::{AnalysisResult, ModerationResult};

#[derive(Debug, Error)]
pub enum LlmError {
    #[error("Request failed: {0}")]
    RequestFailed(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
}

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Query error: {0}")]
    QueryError(String),

    #[error("Not found: {0}")]
    NotFound(String),
}

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn analyze(&self, message: &str) -> Result<AnalysisResult, LlmError>;
    async fn explain(&self, message: &str, score: f32) -> Result<String, LlmError>;
}

#[async_trait]
pub trait ResultStore: Send + Sync {
    async fn save(&self, result: &ModerationResult) -> Result<(), StoreError>;
    async fn get(&self, id: Uuid) -> Result<Option<ModerationResult>, StoreError>;
}
