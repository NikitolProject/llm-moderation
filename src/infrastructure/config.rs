use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub vllm_base_url: String,
    pub vllm_model: String,
    pub api_key: String,
    pub danger_threshold: f32,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            vllm_base_url: env::var("VLLM_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:8000".to_string()),
            vllm_model: env::var("VLLM_MODEL")
                .unwrap_or_else(|_| "GPT-OSS:20b".to_string()),
            api_key: env::var("API_KEY")
                .expect("API_KEY environment variable is required"),
            danger_threshold: env::var("DANGER_THRESHOLD")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(65.0),
        }
    }
}
