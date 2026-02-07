use axum::{extract::State, http::StatusCode, Json};
use models::{ApiError, ApiErrorResponse, ModerationRequest, ModerationResponse};
use validator::Validate;

use crate::application::ApplicationError;
use crate::presentation::state::AppState;

#[utoipa::path(
    post,
    path = "/api/v1/moderate",
    tag = "moderation",
    request_body = ModerationRequest,
    responses(
        (status = 200, description = "Moderation result", body = ModerationResponse),
        (status = 400, description = "Validation error", body = ApiErrorResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error", body = ApiErrorResponse)
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn moderate(
    State(state): State<AppState>,
    Json(request): Json<ModerationRequest>,
) -> Result<Json<ModerationResponse>, (StatusCode, Json<ApiErrorResponse>)> {
    if let Err(errors) = request.validate() {
        let error = ApiError::Validation(errors.to_string());
        return Err((StatusCode::BAD_REQUEST, Json(error.to_response())));
    }

    let result = state
        .moderation_service
        .moderate(request.message)
        .await
        .map_err(|e| map_application_error(e))?;

    let threshold = state.moderation_service.danger_threshold();
    let requires_review = result.requires_review(threshold);

    Ok(Json(ModerationResponse {
        id: result.id.as_uuid(),
        danger_score: result.danger_score.value(),
        categories: result.categories,
        requires_review,
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
