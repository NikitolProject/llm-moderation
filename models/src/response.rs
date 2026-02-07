use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum DangerCategory {
    RadicalPositions,
    AdvertisingSpam,
    Doxxing,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ModerationResponse {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    #[schema(example = 75.5)]
    pub danger_score: f32,
    pub categories: Vec<DangerCategory>,
    #[schema(example = true)]
    pub requires_review: bool,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ReasonResponse {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    #[schema(example = 82.3)]
    pub danger_score: f32,
    #[schema(example = "The message contains discriminatory language targeting...")]
    pub reason: String,
    pub categories: Vec<DangerCategory>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct HealthResponse {
    #[schema(example = "ok")]
    pub status: String,
}
