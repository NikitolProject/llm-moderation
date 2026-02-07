use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use models::{ApiError, ApiErrorResponse, ReasonResponse};
use uuid::Uuid;

use crate::application::ApplicationError;
use crate::presentation::state::AppState;

#[utoipa::path(
    post,
    path = "/api/v1/moderate/{id}/reason",
    tag = "moderation",
    params(
        ("id" = Uuid, Path, description = "Moderation result ID")
    ),
    responses(
        (status = 200, description = "Reason for flagged content", body = ReasonResponse),
        (status = 400, description = "Reason not available (score below threshold)", body = ApiErrorResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Moderation result not found", body = ApiErrorResponse),
        (status = 500, description = "Internal server error", body = ApiErrorResponse)
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn get_reason(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ReasonResponse>, (StatusCode, Json<ApiErrorResponse>)> {
    let (result, reason) = state
        .moderation_service
        .get_reason(id)
        .await
        .map_err(|e| map_application_error(e))?;

    Ok(Json(ReasonResponse {
        id: result.id.as_uuid(),
        danger_score: result.danger_score.value(),
        reason,
        categories: result.categories,
    }))
}

fn map_application_error(error: ApplicationError) -> (StatusCode, Json<ApiErrorResponse>) {
    match error {
        ApplicationError::Llm(e) => {
            let api_error = ApiError::LlmError(e.to_string());
            (StatusCode::SERVICE_UNAVAILABLE, Json(api_error.to_response()))
        }
        ApplicationError::Store(e) => {
            let api_error = ApiError::DatabaseError(e.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, Json(api_error.to_response()))
        }
        ApplicationError::NotFound(id) => {
            let api_error = ApiError::NotFound(format!("Moderation result {} not found", id));
            (StatusCode::NOT_FOUND, Json(api_error.to_response()))
        }
        ApplicationError::ReasonNotAvailable { score, threshold } => {
            let api_error = ApiError::ReasonNotAvailable(score, threshold);
            (StatusCode::BAD_REQUEST, Json(api_error.to_response()))
        }
    }
}
