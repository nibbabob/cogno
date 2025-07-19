//! llm_api.rs
//!
//! Interacts with an external LLM for cognitive appraisal.

use crate::cognitive_appraisal::OccEmotion;
use reqwest::Client;
use serde_json::Value;
use std::env;

/// Calls the LLM to appraise the emotion in a user's prompt.
pub async fn call_llm_for_appraisal(user_prompt: &str) -> Result<OccEmotion, Box<dyn std::error::Error>> {
    println!("📞 Calling LLM API for cognitive appraisal...");

    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set in environment");
    let api_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
        api_key
    );

    let client = Client::new();

    // FIX: A much stricter and more detailed prompt.
    let prompt_text = format!(
        r#"Analyze the user's text and respond with a single, clean JSON object representing the primary emotion based on the OCC model.

        **JSON Schema:**
        - The top-level object must have two keys: "emotion" (string) and "details" (object).
        - The "emotion" value must be one of the PascalCase enum variants (e.g., "Pride", "Joy", "Anger").
        - The "details" object's structure depends on the emotion.

        **Required Structures for "details":**
        - Joy: {{"intensity": number, "focus": string}}
        - Pride: {{"intensity": number, "target": string}}
        - Gratitude: {{"intensity": number, "agent": string}}
        - Distress: {{"intensity": number, "focus": string}}
        - Anger: {{"intensity": number, "target": string}}
        - Fear: {{"likelihood": number, "event": string}}
        - Hope: {{"likelihood": number, "event": string}}
        - Shame: {{"intensity": number, "action": string}}
        - Disappointment: {{"intensity": number, "expectation": string}}
        - Satisfaction: {{"intensity": number, "goal": string}}
        - Relief: {{"intensity": number}}
        - Neutral: {{}}

        **Example Response for "I'm so happy I won the game!":**
        {{"emotion": "Joy", "details": {{"intensity": 0.8, "focus": "winning the game"}}}}

        **User Text:**
        "{}"

        Respond only with the JSON object."#,
        user_prompt
    );

    let response = client
        .post(&api_url)
        .json(&serde_json::json!({
            "contents": [{
                "parts": [{
                    "text": prompt_text
                }]
            }]
        }))
        .send()
        .await?;

    if response.status().is_success() {
        let body: Value = response.json().await?;
        println!("📄 Raw API Response Body: {}", serde_json::to_string_pretty(&body)?);

        let text_content = body
            .get("candidates")
            .and_then(|c| c.get(0))
            .and_then(|c| c.get("content"))
            .and_then(|p| p.get("parts"))
            .and_then(|p| p.get(0))
            .and_then(|p| p.get("text"))
            .and_then(|t| t.as_str())
            .unwrap_or_default();
            
        let cleaned_text = text_content
            .trim()
            .trim_start_matches("```json")
            .trim_end_matches("```")
            .trim();

        if cleaned_text.is_empty() {
             return Err("LLM response was empty or had an invalid structure.".into());
        }

        serde_json::from_str(cleaned_text).map_err(|e| {
            let error_message = format!("Failed to parse LLM JSON response: {}. Cleaned content: '{}'", e, cleaned_text);
            error_message.into()
        })
    } else {
        let status = response.status();
        let error_body = response.text().await?;
        let error_message = format!("LLM API request failed. Status: {}. Body: {}", status, error_body);
        Err(error_message.into())
    }
}