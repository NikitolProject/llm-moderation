use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ApiErrorResponse {
    #[schema(example = "validation_error")]
    pub code: String,
    #[schema(example = "Invalid request parameters")]
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Reason not available: danger score {0} is below threshold {1}")]
    ReasonNotAvailable(f32, f32),
    #[error("LLM service error: {0}")]
    LlmError(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Internal server error: {0}")]
    Internal(String),
}

impl ApiError {
    pub fn to_response(&self) -> ApiErrorResponse {
        match self {
            ApiError::Validation(msg) => ApiErrorResponse {
                code: "validation_error".to_string(),
                message: msg.clone(),
                details: None,
            },
            ApiError::NotFound(msg) => ApiErrorResponse {
                code: "not_found".to_string(),
                message: msg.clone(),
                details: None,
            },
            ApiError::Unauthorized => ApiErrorResponse {
                code: "unauthorized".to_string(),
                message: "Invalid or missing API key".to_string(),
                details: None,
            },
            ApiError::ReasonNotAvailable(score, threshold) => ApiErrorResponse {
                code: "reason_not_available".to_string(),
                message: format!(
                    "Reason is only available for messages with danger score above {}%",
                    threshold
                ),
                details: Some(format!("Current score: {}%", score)),
            },
            ApiError::LlmError(msg) => ApiErrorResponse {
                code: "llm_error".to_string(),
                message: "Failed to analyze content".to_string(),
                details: Some(msg.clone()),
            },
            ApiError::DatabaseError(msg) => ApiErrorResponse {
                code: "database_error".to_string(),
                message: "Database operation failed".to_string(),
                details: Some(msg.clone()),
            },
            ApiError::Internal(msg) => ApiErrorResponse {
                code: "internal_error".to_string(),
                message: "An internal error occurred".to_string(),
                details: Some(msg.clone()),
            },
        }
    }
}
