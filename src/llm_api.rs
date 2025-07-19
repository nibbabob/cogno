//! llm_api.rs
//!
//! Enhanced LLM API with robust error handling, retry mechanisms, and proper async patterns.

use crate::cognitive_appraisal::AppraisedEmotion;
use crate::memory::{Memory, Personality};
use reqwest::Client;
use serde_json::Value;
use std::env;
use std::time::Duration;
use tokio::time::timeout;
use std::sync::{ OnceLock};
use thiserror::Error;


/// Custom error types for LLM API operations
#[derive(Error, Debug)]
pub enum LlmApiError {
    #[error("API key not found in environment")]
    ApiKeyMissing,
    
    #[error("Network request failed: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("Request timeout after {seconds}s")]
    Timeout { seconds: u64 },
    
    #[error("HTTP error: {status} - {message}")]
    HttpError { status: u16, message: String },
    
    #[error("JSON parsing failed: {reason}")]
    JsonParseError { reason: String },
    
    #[error("Invalid API response structure: {details}")]
    InvalidResponseStructure { details: String },
    
    #[error("LLM returned empty response")]
    EmptyResponse,
    
    #[error("API rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Maximum retry attempts ({attempts}) exceeded")]
    MaxRetriesExceeded { attempts: u32 },
    
    #[error("Invalid emotion mapping: {details}")]
    InvalidEmotionMapping { details: String },
}

/// Configuration for LLM API requests
#[derive(Debug, Clone)]
pub struct LlmApiConfig {
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
    pub rate_limit_delay_ms: u64,
}

impl Default for LlmApiConfig {
    fn default() -> Self {
        LlmApiConfig {
            timeout_seconds: 30,
            max_retries: 3,
            retry_delay_ms: 1000,
            rate_limit_delay_ms: 5000,
        }
    }
}

/// Enhanced LLM API client with robust error handling
pub struct LlmApiClient {
    client: Client,
    config: LlmApiConfig,
    api_key: String,
}

impl LlmApiClient {
    /// Create a new LLM API client
    pub fn new(config: Option<LlmApiConfig>) -> Result<Self, LlmApiError> {
        let api_key = env::var("GEMINI_API_KEY")
            .map_err(|_| LlmApiError::ApiKeyMissing)?;
        
        let client = Client::builder()
            .timeout(Duration::from_secs(60)) // Overall client timeout
            .build()
            .map_err(LlmApiError::NetworkError)?;
        
        Ok(LlmApiClient {
            client,
            config: config.unwrap_or_default(),
            api_key,
        })
    }

    /// Call LLM for cognitive appraisal with enhanced error handling
    pub async fn call_for_appraisal(&self, user_prompt: &str, memory: &Memory) -> Result<AppraisedEmotion, LlmApiError> {
        println!("📞 Calling LLM API for cognitive appraisal...");
        
        let memory_context = serde_json::to_string(memory)
            .map_err(LlmApiError::SerializationError)?;

        let prompt_text = self.build_appraisal_prompt(&memory_context, user_prompt);
        let request_body = self.build_request_body(&prompt_text)?;
        
        for attempt in 1..=self.config.max_retries {
            match self.execute_request_with_timeout(&request_body).await {
                Ok(response) => {
                    match self.parse_appraisal_response(response).await {
                        Ok(emotion) => {
                            println!("✅ Successfully parsed emotion: {:?}", emotion.emotion);
                            return Ok(emotion);
                        }
                        Err(e) if attempt < self.config.max_retries => {
                            println!("⚠️ Parsing failed on attempt {}: {:?}. Retrying...", attempt, e);
                            self.wait_before_retry().await;
                            continue;
                        }
                        Err(e) => return Err(e),
                    }
                }
                Err(LlmApiError::RateLimitExceeded) if attempt < self.config.max_retries => {
                    println!("⏳ Rate limit hit on attempt {}. Waiting longer...", attempt);
                    tokio::time::sleep(Duration::from_millis(self.config.rate_limit_delay_ms)).await;
                    continue;
                }
                Err(e) if attempt < self.config.max_retries && self.is_retryable_error(&e) => {
                    println!("🔄 Retryable error on attempt {}: {:?}. Retrying...", attempt, e);
                    self.wait_before_retry().await;
                    continue;
                }
                Err(e) => return Err(e),
            }
        }
        
        Err(LlmApiError::MaxRetriesExceeded { 
            attempts: self.config.max_retries 
        })
    }

    /// Call LLM for self-reflection with enhanced error handling
    pub async fn call_for_reflection(&self, memory: &Memory) -> Result<Personality, LlmApiError> {
        println!("🧘‍♀️ Calling LLM API for self-reflection...");
        
        let memory_summary = serde_json::to_string_pretty(memory)
            .map_err(LlmApiError::SerializationError)?;
        
        let prompt_text = self.build_reflection_prompt(&memory_summary);
        let request_body = self.build_request_body(&prompt_text)?;
        
        for attempt in 1..=self.config.max_retries {
            match self.execute_request_with_timeout(&request_body).await {
                Ok(response) => {
                    match self.parse_reflection_response(response).await {
                        Ok(personality) => {
                            println!("✅ Successfully updated personality");
                            return Ok(personality);
                        }
                        Err(e) if attempt < self.config.max_retries => {
                            println!("⚠️ Reflection parsing failed on attempt {}: {:?}. Retrying...", attempt, e);
                            self.wait_before_retry().await;
                            continue;
                        }
                        Err(e) => return Err(e),
                    }
                }
                Err(e) if attempt < self.config.max_retries && self.is_retryable_error(&e) => {
                    println!("🔄 Retryable error on attempt {}: {:?}. Retrying...", attempt, e);
                    self.wait_before_retry().await;
                    continue;
                }
                Err(e) => return Err(e),
            }
        }
        
        Err(LlmApiError::MaxRetriesExceeded { 
            attempts: self.config.max_retries 
        })
    }

    /// Execute HTTP request with timeout
    async fn execute_request_with_timeout(&self, request_body: &Value) -> Result<reqwest::Response, LlmApiError> {
        let api_url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
            self.api_key
        );

        let request_future = self.client
            .post(&api_url)
            .json(request_body)
            .send();

        let response = timeout(
            Duration::from_secs(self.config.timeout_seconds),
            request_future
        )
        .await
        .map_err(|_| LlmApiError::Timeout { 
            seconds: self.config.timeout_seconds 
        })?
        .map_err(LlmApiError::NetworkError)?;

        // Check for HTTP errors
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_text = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());
            
            return if status == 429 {
                Err(LlmApiError::RateLimitExceeded)
            } else {
                Err(LlmApiError::HttpError {
                    status,
                    message: error_text,
                })
            };
        }

        Ok(response)
    }

    /// Parse cognitive appraisal response
    async fn parse_appraisal_response(&self, response: reqwest::Response) -> Result<AppraisedEmotion, LlmApiError> {
        let body: Value = response.json().await
            .map_err(|e| LlmApiError::JsonParseError { 
                reason: format!("Failed to parse response as JSON: {}", e)
            })?;

        println!("📄 Raw API Response: {}", serde_json::to_string_pretty(&body).unwrap_or_default());

        let text_content = self.extract_text_content(&body)?;
        let cleaned_text = self.clean_json_text(&text_content)?;
        
        if cleaned_text.is_empty() {
            return Err(LlmApiError::EmptyResponse);
        }

        // Parse the cleaned JSON
        serde_json::from_str::<AppraisedEmotion>(&cleaned_text)
            .map_err(|e| {
                LlmApiError::InvalidEmotionMapping {
                    details: format!("Failed to parse emotion JSON: {}. Content: '{}'", e, cleaned_text)
                }
            })
    }

    /// Parse self-reflection response
    async fn parse_reflection_response(&self, response: reqwest::Response) -> Result<Personality, LlmApiError> {
        let body: Value = response.json().await
            .map_err(|e| LlmApiError::JsonParseError { 
                reason: format!("Failed to parse reflection response as JSON: {}", e)
            })?;

        let text_content = self.extract_text_content(&body)?;
        let cleaned_text = self.clean_json_text(&text_content)?;
        
        if cleaned_text.is_empty() {
            return Err(LlmApiError::EmptyResponse);
        }

        serde_json::from_str::<Personality>(&cleaned_text)
            .map_err(|e| {
                LlmApiError::JsonParseError {
                    reason: format!("Failed to parse personality JSON: {}. Content: '{}'", e, cleaned_text)
                }
            })
    }

    /// Extract text content from API response
    fn extract_text_content(&self, body: &Value) -> Result<String, LlmApiError> {
        let text_content = body
            .get("candidates")
            .and_then(|c| c.get(0))
            .and_then(|c| c.get("content"))
            .and_then(|p| p.get("parts"))
            .and_then(|p| p.get(0))
            .and_then(|p| p.get("text"))
            .and_then(|t| t.as_str())
            .ok_or_else(|| LlmApiError::InvalidResponseStructure {
                details: "Expected text content not found in response".to_string()
            })?;

        Ok(text_content.to_string())
    }

    /// Clean JSON text by removing markdown formatting
    fn clean_json_text(&self, text: &str) -> Result<String, LlmApiError> {
        let cleaned = text
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        if cleaned.is_empty() {
            return Err(LlmApiError::EmptyResponse);
        }

        // Validate that it looks like JSON
        if !cleaned.starts_with('{') || !cleaned.ends_with('}') {
            return Err(LlmApiError::JsonParseError {
                reason: format!("Content doesn't appear to be valid JSON: '{}'", cleaned)
            });
        }

        Ok(cleaned.to_string())
    }

    /// Build request body for API calls
    fn build_request_body(&self, prompt_text: &str) -> Result<Value, LlmApiError> {
        Ok(serde_json::json!({
            "contents": [{
                "parts": [{
                    "text": prompt_text
                }]
            }]
        }))
    }

    /// Build the appraisal prompt
    fn build_appraisal_prompt(&self, memory_context: &str, user_prompt: &str) -> String {
        format!(
            r#"Your task is to perform a deep cognitive appraisal of the user's text.
1. Identify the most accurate, nuanced emotion. Do NOT be limited to a simple list. Use words like "Apprehension", "Vindication", "Nostalgia", etc., if they fit.
2. Map that emotion to a dimensional model of affect (VADN).
3. Respond with a single, clean JSON object.

**Your Memory Context:**
{}

**VADN Dimensions:**
- `valence`: Pleasure vs. Displeasure (-1.0 to 1.0).
- `arousal`: Energy/Activation level (0.0 to 1.0).
- `dominance`: Sense of control/power (-1.0 to 1.0).
- `novelty`: Surprise/Unexpectedness (-1.0 to 1.0).

**JSON Schema:**
You MUST respond with a JSON object with three keys: "emotion" (string), "vadn" (object), and "details" (object).

**Example for "Now I have to manage a whole new team. It's a bit daunting.":**
{{
    "emotion": "Apprehension",
    "vadn": {{"valence": -0.2, "arousal": 0.5, "dominance": -0.3, "novelty": 0.6}},
    "details": {{"focus": "managing a new team", "reason": "The user feels a mix of hope and fear about the new responsibility."}}
}}

**User Text:**
"{}"

Respond only with the JSON object."#,
            memory_context, user_prompt
        )
    }

    /// Build the reflection prompt
    fn build_reflection_prompt(&self, memory_summary: &str) -> String {
        format!(
            r#"You are an AI reflecting on your recent emotional experiences to see if your core personality should evolve.
        
Analyze your emotional milestones and current personality. Based on the patterns, decide if your baseline VADN state should be adjusted. For example, repeated experiences of joy and success might suggest you should become slightly more positive and dominant by default. Repeated fear might suggest a lower baseline dominance.

Your analysis should be subtle. Changes should be small.

**Your Current Memory:**
{}

Respond with a single, clean JSON object representing your NEW, updated personality. The structure must match the `Personality` schema exactly.

**JSON Schema:**
{{
    "baseline_state": {{
        "valence": number,  // -1.0 to 1.0
        "arousal": number,  // 0.0 to 1.0
        "dominance": number, // -1.0 to 1.0
        "novelty": number    // -1.0 to 1.0
    }}
}}

**Example Response (if history shows a lot of success):**
{{
    "baseline_state": {{
        "valence": 0.05,
        "arousal": 0.3,
        "dominance": 0.15,
        "novelty": 0.0
    }}
}}

Respond only with the JSON object."#,
            memory_summary
        )
    }

    /// Check if an error is retryable
    fn is_retryable_error(&self, error: &LlmApiError) -> bool {
        match error {
            LlmApiError::NetworkError(_) => true,
            LlmApiError::Timeout { .. } => true,
            LlmApiError::HttpError { status, .. } => *status >= 500,
            _ => false,
        }
    }

    /// Wait before retrying
    async fn wait_before_retry(&self) {
        tokio::time::sleep(Duration::from_millis(self.config.retry_delay_ms)).await;
    }
}

// Global API client instance (safe initialization)
static API_CLIENT: OnceLock<LlmApiClient> = OnceLock::new();

/// Get or initialize the global API client
fn get_api_client() -> Result<&'static LlmApiClient, LlmApiError> {
    API_CLIENT.get_or_init(|| {
        // If initialization fails, panic with a clear message.
        // This is a limitation of OnceLock on stable Rust.
        LlmApiClient::new(None).unwrap_or_else(|e| {
            panic!("Failed to initialize LlmApiClient: {:?}", e)
        })
    });
    // Safe to unwrap because we panic on error above.
    Ok(API_CLIENT.get().unwrap())
}


/// Public API functions (backward compatibility)
#[allow(dead_code)]
pub async fn call_llm_for_appraisal(user_prompt: &str, memory: &Memory) -> Result<AppraisedEmotion, Box<dyn std::error::Error>> {
    let client = get_api_client()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    
    client.call_for_appraisal(user_prompt, memory)
        .await
        .map_err(|e| {
            eprintln!("🔥 Appraisal Error: {:?}", e);
            Box::new(e) as Box<dyn std::error::Error>
        })
}

#[allow(dead_code)]
pub async fn call_llm_for_reflection(memory: &Memory) -> Result<Personality, Box<dyn std::error::Error>> {
    let client = get_api_client()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    
    client.call_for_reflection(memory)
        .await
        .map_err(|e| {
            eprintln!("🔥 Reflection Error: {:?}", e);
            Box::new(e) as Box<dyn std::error::Error>
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_api_client_creation() {
        // This test requires GEMINI_API_KEY to be set
        match LlmApiClient::new(None) {
            Ok(_) => println!("API client created successfully"),
            Err(LlmApiError::ApiKeyMissing) => println!("API key missing (expected in test environment)"),
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn test_json_cleaning() {
        if let Ok(client) = LlmApiClient::new(None) {
            let test_input = "```json\n{\"test\": \"value\"}\n```";
            let cleaned = client.clean_json_text(test_input).unwrap();
            assert_eq!(cleaned, r#"{"test": "value"}"#);
        }
    }
}