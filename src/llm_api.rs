//! llm_api.rs
//!
//! Interacts with an external LLM for cognitive appraisal and self-reflection.
//! Updated to use the recommended API key header and generationConfig for JSON output.

use crate::cognitive_appraisal::AppraisedEmotion;
use crate::memory::{Memory, Personality}; // Import Personality
use reqwest::{header, Client};
use serde_json::Value;
use std::env;
use std::time::Duration;

// --- Constants for Configuration ---
const API_BASE_URL: &str = "[https://generativelanguage.googleapis.com/v1beta/models/](https://generativelanguage.googleapis.com/v1beta/models/)";
const MODEL_NAME: &str = "gemini-1.5-pro"; // Using a more capable model

/// **MODIFIED**: Creates a configured HTTP client.
/// It now reads the API key and sets it as a default header (`x-goog-api-key`)
/// for all requests made with this client. This is the modern, recommended approach.
fn create_http_client() -> Result<Client, Box<dyn std::error::Error>> {
    let api_key = env::var("GEMINI_API_KEY").map_err(|_| "GEMINI_API_KEY environment variable not set")?;

    let mut headers = header::HeaderMap::new();
    // Create a HeaderValue from the API key.
    let mut auth_value = header::HeaderValue::from_str(&api_key)?;
    // Mark the header as sensitive to prevent it from being logged.
    auth_value.set_sensitive(true);
    // Insert the `x-goog-api-key` header.
    headers.insert("x-goog-api-key", auth_value);

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .default_headers(headers) // Set the default headers for the client
        .build()?;

    Ok(client)
}

/// Test the API connection with a simple request.
pub async fn test_api_connection() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Testing API connection...");

    // **MODIFIED**: The API URL no longer contains the key.
    let api_url = format!("{}{}:generateContent", API_BASE_URL, MODEL_NAME);

    // **MODIFIED**: The client is now created with the auth header built-in.
    let client = create_http_client()?;

    let test_request = serde_json::json!({
        "contents": [{
            "parts": [{
                "text": "Test connection. Respond with just 'OK'."
            }]
        }]
    });

    let response = client
        .post(&api_url)
        .header(header::CONTENT_TYPE, "application/json") // Explicitly set content type
        .json(&test_request)
        .send()
        .await?;

    if response.status().is_success() {
        println!("✅ API connection successful");
        Ok(())
    } else {
        let status = response.status();
        let error_body = response.text().await.unwrap_or_else(|_| "Unable to read error body".to_string());
        Err(format!("API test failed. Status: {}. Body: {}", status, error_body).into())
    }
}

pub async fn call_llm_for_appraisal(user_prompt: &str, memory: &Memory) -> Result<AppraisedEmotion, Box<dyn std::error::Error>> {
    println!("📞 Calling LLM API for cognitive appraisal...");

    // **MODIFIED**: The API URL is cleaner and no longer contains the key.
    let api_url = format!("{}{}:generateContent", API_BASE_URL, MODEL_NAME);

    // **MODIFIED**: The client handles authentication automatically via default headers.
    let client = create_http_client()?;
    let memory_context = serde_json::to_string(memory).unwrap_or_else(|_| "{}".to_string());

    let prompt_text = format!(
        r#"Analyze the emotional content of this user message and respond with JSON only.

User message: "{}"

Memory context: {}

Respond with this exact JSON structure:
{{
  "emotion": "happiness",
  "vadn": {{"valence": 0.7, "arousal": 0.5, "dominance": 0.3, "novelty": 0.2}},
  "details": {{"focus": "user seems happy", "reason": "positive language detected"}}
}}

Values should be between -1.0 and 1.0 for valence/dominance/novelty, and 0.0 to 1.0 for arousal.
Respond ONLY with valid JSON, no other text."#,
        user_prompt, memory_context
    );

    println!("🔄 Sending request to API...");

    // **MODIFIED**: Added `generationConfig` to request a JSON response directly.
    // This makes parsing much more reliable.
    let request_body = serde_json::json!({
        "contents": [{
            "parts": [{
                "text": prompt_text
            }]
        }],
        "generationConfig": {
            "response_mime_type": "application/json"
        }
    });

    let response = match client
        .post(&api_url)
        .header(header::CONTENT_TYPE, "application/json")
        .json(&request_body)
        .send()
        .await {
        Ok(resp) => resp,
        Err(e) => {
            return Err(format!("HTTP request failed: {}. Check your internet connection.", e).into());
        }
    };

    println!("📨 Received response with status: {}", response.status());

    if response.status().is_success() {
        let body: Value = response.json().await?;
        println!("📄 Raw API Response: {}", serde_json::to_string_pretty(&body)?);

        // **MODIFIED**: The path to the text is slightly different for JSON responses.
        // It's directly in `parts[0]`, not `parts[0].text`.
        // We now expect the model to return a JSON object directly.
        let json_content = body
            .get("candidates")
            .and_then(|c| c.get(0))
            .and_then(|c| c.get("content"))
            .and_then(|p| p.get("parts"))
            .and_then(|p| p.get(0))
            .and_then(|p| p.get("text")) // The model still wraps the JSON in the "text" field
            .ok_or("Could not find 'text' field in the API response.")?;

        // The response should be a string containing JSON.
        let json_string = json_content.as_str().ok_or("Expected text field to be a string.")?;
        
        println!("📝 Extracted JSON string: {}", json_string);

        // We parse the string into our AppraisedEmotion struct.
        match serde_json::from_str(json_string) {
            Ok(emotion) => {
                println!("✅ Successfully parsed emotion from JSON response.");
                Ok(emotion)
            },
            Err(e) => {
                println!("❌ JSON parsing failed: {}. The model did not return valid JSON despite the configuration.", e);
                create_fallback_emotion(user_prompt)
            }
        }
    } else {
        // Error handling remains largely the same, but updated for clarity.
        let status = response.status();
        let error_body = response.text().await.unwrap_or_else(|_| "Unable to read error body".to_string());
        
        println!("❌ API request failed: Status {}", status);
        println!("📄 Error body: {}", error_body);
        
        let error_msg = match status.as_u16() {
            400 => format!("Bad request. Check the request body. Details: {}", error_body),
            401 | 403 => "Authentication failed. Please check your GEMINI_API_KEY and its permissions.".to_string(),
            429 => "Rate limit exceeded. Please wait and try again.".to_string(),
            500..=599 => "Server error. The API service may be temporarily unavailable.".to_string(),
            _ => format!("HTTP error {}: {}", status, error_body)
        };
        
        Err(error_msg.into())
    }
}

/// Create a fallback emotion when LLM parsing fails. (No changes needed here)
fn create_fallback_emotion(user_prompt: &str) -> Result<AppraisedEmotion, Box<dyn std::error::Error>> {
    println!("🔧 Creating fallback emotion based on keyword analysis...");
    
    let prompt_lower = user_prompt.to_lowercase();
    
    let (emotion, valence, arousal, dominance, novelty) = if prompt_lower.contains("happy") || prompt_lower.contains("great") {
        ("happiness", 0.7, 0.5, 0.3, 0.0)
    } else if prompt_lower.contains("sad") || prompt_lower.contains("upset") {
        ("sadness", -0.6, 0.3, -0.2, 0.0)
    } else if prompt_lower.contains("angry") || prompt_lower.contains("mad") {
        ("anger", -0.5, 0.8, 0.4, 0.1)
    } else if prompt_lower.contains("scared") || prompt_lower.contains("afraid") {
        ("fear", -0.4, 0.7, -0.5, 0.3)
    } else if prompt_lower.contains("excited") || prompt_lower.contains("amazing") {
        ("excitement", 0.8, 0.9, 0.2, 0.4)
    } else {
        ("neutral", 0.0, 0.3, 0.0, 0.0)
    };

    let fallback_emotion = AppraisedEmotion {
        emotion: emotion.to_string(),
        vadn: crate::cognitive_appraisal::AffectiveStateChange {
            valence,
            arousal,
            dominance,
            novelty,
        },
        details: serde_json::json!({
            "focus": "fallback analysis",
            "reason": format!("Keyword-based analysis of: {}", user_prompt)
        }),
    };

    println!("🎯 Created fallback emotion: {:?}", fallback_emotion);
    Ok(fallback_emotion)
}

/// Calls the LLM to reflect on its own emotional history and suggest a personality change.
pub async fn call_llm_for_reflection(memory: &Memory) -> Result<Personality, Box<dyn std::error::Error>> {
    println!("🧘‍♀️ Calling LLM API for self-reflection...");

    let api_url = format!("{}{}:generateContent", API_BASE_URL, MODEL_NAME);
    let client = create_http_client()?;
    let memory_summary = serde_json::to_string_pretty(&memory)?;

    let prompt_text = format!(
        r#"Analyze the emotional history and suggest a new personality baseline.

Memory: {}

Respond with this exact JSON structure:
{{
  "baseline_state": {{
    "valence": 0.05,
    "arousal": 0.3,
    "dominance": 0.15,
    "novelty": 0.0
  }}
}}

Values: valence/dominance/novelty (-1.0 to 1.0), arousal (0.0 to 1.0).
Respond ONLY with valid JSON."#,
        memory_summary
    );

    // **MODIFIED**: Also using generationConfig here for reliable JSON.
    let request_body = serde_json::json!({
        "contents": [{ "parts": [{ "text": prompt_text }] }],
        "generationConfig": {
            "response_mime_type": "application/json"
        }
    });

    let response = client
        .post(&api_url)
        .header(header::CONTENT_TYPE, "application/json")
        .json(&request_body)
        .send()
        .await?;

    if response.status().is_success() {
        let body: Value = response.json().await?;
        let json_content = body.get("candidates").and_then(|c| c.get(0)).and_then(|c| c.get("content")).and_then(|p| p.get("parts")).and_then(|p| p.get(0)).and_then(|p| p.get("text")).ok_or("Could not find 'text' in response")?;
        let json_string = json_content.as_str().ok_or("Expected text to be a string")?;
        
        serde_json::from_str(json_string).map_err(|e| {
            println!("❌ Failed to parse personality reflection from LLM: {}", e);
            e.into()
        })
    } else {
        Err(format!("LLM reflection API request failed: {}", response.status()).into())
    }
}
