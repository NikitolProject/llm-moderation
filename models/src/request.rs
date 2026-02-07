use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct ModerationRequest {
    #[validate(length(min = 1, max = 10000, message = "Message must be between 1 and 10000 characters"))]
    #[schema(example = "Sample text to moderate")]
    pub message: String,
}
