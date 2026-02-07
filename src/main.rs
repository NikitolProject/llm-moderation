use std::net::SocketAddr;
use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use llm_moderation::application::ModerationService;
use llm_moderation::infrastructure::{AppConfig, PostgresModerationRepository, VllmClient};
use llm_moderation::presentation::{build_router, AppState};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "llm_moderation=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppConfig::from_env();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable is required");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    tracing::info!("Connected to PostgreSQL");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    tracing::info!("Migrations completed");

    let llm_client = Arc::new(VllmClient::new(
        config.vllm_base_url.clone(),
        config.vllm_model.clone(),
    ));

    let result_store = Arc::new(PostgresModerationRepository::new(pool));

    let moderation_service = ModerationService::new(
        llm_client,
        result_store,
        config.danger_threshold,
    );

    let state = AppState::new(config, moderation_service);
    let router = build_router(state);

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .expect("PORT must be a valid number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await.expect("Failed to bind to address");

    tracing::info!("Server listening on http://{}", addr);

    axum::serve(listener, router)
        .await
        .expect("Server failed");
}
