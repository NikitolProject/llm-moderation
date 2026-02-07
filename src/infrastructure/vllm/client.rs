use async_trait::async_trait;
use models::DangerCategory;
use reqwest::Client;
use serde::Deserialize;

use crate::domain::{AnalysisResult, LlmClient, LlmError};

use super::models::{ChatCompletionRequest, ChatCompletionResponse, ChatMessage};
use super::prompts::{
    format_analysis_prompt, format_explanation_prompt, ANALYSIS_SYSTEM_PROMPT,
    EXPLANATION_SYSTEM_PROMPT,
};

pub struct VllmClient {
    client: Client,
    base_url: String,
    model: String,
}

impl VllmClient {
    pub fn new(base_url: String, model: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            model,
        }
    }

    async fn chat_completion(&self, messages: Vec<ChatMessage>) -> Result<String, LlmError> {
        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages,
            temperature: 0.1,
            max_tokens: 1024,
        };

        let url = format!("{}/v1/chat/completions", self.base_url);

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| LlmError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(LlmError::RequestFailed(format!("HTTP {}: {}", status, body)));
        }

        let completion: ChatCompletionResponse = response
            .json()
            .await
            .map_err(|e| LlmError::InvalidResponse(e.to_string()))?;

        completion
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| LlmError::InvalidResponse("No choices in response".to_string()))
    }
}

#[derive(Debug, Deserialize)]
struct LlmAnalysisResponse {
    danger_score: f32,
    categories: Vec<String>,
}

#[async_trait]
impl LlmClient for VllmClient {
    async fn analyze(&self, message: &str) -> Result<AnalysisResult, LlmError> {
        let messages = vec![
            ChatMessage::system(ANALYSIS_SYSTEM_PROMPT),
            ChatMessage::user(format_analysis_prompt(message)),
        ];

        let response = self.chat_completion(messages).await?;

        let analysis: LlmAnalysisResponse = serde_json::from_str(&response).map_err(|e| {
            LlmError::InvalidResponse(format!("Failed to parse JSON: {}. Response: {}", e, response))
        })?;

        let categories = analysis
            .categories
            .iter()
            .filter_map(|s| match s.as_str() {
                "radical_positions" => Some(DangerCategory::RadicalPositions),
                "advertising_spam" => Some(DangerCategory::AdvertisingSpam),
                "doxxing" => Some(DangerCategory::Doxxing),
                _ => None,
            })
            .collect();

        Ok(AnalysisResult {
            danger_score: analysis.danger_score,
            categories,
        })
    }

    async fn explain(&self, message: &str, score: f32) -> Result<String, LlmError> {
        let messages = vec![
            ChatMessage::system(EXPLANATION_SYSTEM_PROMPT),
            ChatMessage::user(format_explanation_prompt(message, score)),
        ];

        self.chat_completion(messages).await
    }
}
