use thiserror::Error;

use crate::domain::{LlmError, StoreError};

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("LLM error: {0}")]
    Llm(#[from] LlmError),

    #[error("Storage error: {0}")]
    Store(#[from] StoreError),

    #[error("Moderation result not found: {0}")]
    NotFound(String),

    #[error("Reason not available: danger score {score} is below threshold {threshold}")]
    ReasonNotAvailable { score: f32, threshold: f32 },
}
