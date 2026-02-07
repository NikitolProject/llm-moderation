use models::{
    ApiErrorResponse, DangerCategory, HealthResponse, ModerationRequest, ModerationResponse,
    ReasonResponse,
};
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "LLM Moderation API",
        version = "1.0.0",
        description = "REST API for content moderation using LLM"
    ),
    paths(
        crate::presentation::handlers::health::health_check,
        crate::presentation::handlers::moderation::moderate,
        crate::presentation::handlers::reason::get_reason
    ),
    components(
        schemas(
            ModerationRequest,
            ModerationResponse,
            ReasonResponse,
            HealthResponse,
            ApiErrorResponse,
            DangerCategory
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "moderation", description = "Content moderation endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("X-API-Key"))),
            );
        }
    }
}
