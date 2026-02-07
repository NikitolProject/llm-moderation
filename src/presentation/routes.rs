use axum::{middleware, routing::{get, post}, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use super::handlers::{get_reason, health_check, moderate};
use super::middleware::api_key_auth;
use super::openapi::ApiDoc;
use super::state::AppState;

pub fn build_router(state: AppState) -> Router {
    let api_routes = Router::new()
        .route("/moderate", post(moderate))
        .route("/moderate/{id}/reason", post(get_reason))
        .route_layer(middleware::from_fn_with_state(state.clone(), api_key_auth));

    Router::new()
        .route("/health", get(health_check))
        .nest("/api/v1", api_routes)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state)
}
