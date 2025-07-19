//! llm_api.rs
//!
//! Interacts with an external LLM for cognitive appraisal and self-reflection.

use crate::cognitive_appraisal::AppraisedEmotion;
use crate::memory::{Memory, Personality}; // Import Personality
use reqwest::Client;
use serde_json::Value;
use std::env;

// ... (call_llm_for_appraisal function remains the same) ...
pub async fn call_llm_for_appraisal(user_prompt: &str, memory: &Memory) -> Result<AppraisedEmotion, Box<dyn std::error::Error>> {
    println!("📞 Calling LLM API for TRUE cognitive appraisal (mapping any emotion)...");

    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set in environment");
    let api_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
        api_key
    );

    let client = Client::new();
    let memory_context = serde_json::to_string(memory).unwrap_or_else(|_| "{}".to_string());

    // **THE NEW PROMPT**
    // This prompt asks the LLM to perform the conceptual mapping for us.
    let prompt_text = format!(
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
            
        let cleaned_text = text_content.trim().trim_start_matches("```json").trim_end_matches("```").trim();

        if cleaned_text.is_empty() {
             return Err("LLM response was empty.".into());
        }

        serde_json::from_str(cleaned_text).map_err(|e| {
            format!("Failed to parse LLM JSON response: {}. Cleaned content: '{}'", e, cleaned_text).into()
        })
    } else {
        let status = response.status();
        let error_body = response.text().await?;
        Err(format!("LLM API request failed. Status: {}. Body: {}", status, error_body).into())
    }
}


/// **NEW**: Calls the LLM to reflect on its own emotional history and suggest a personality change.
pub async fn call_llm_for_reflection(memory: &Memory) -> Result<Personality, Box<dyn std::error::Error>> {
    println!("🧘‍♀️ Calling LLM API for self-reflection...");

    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set in environment");
    let api_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
        api_key
    );
    let client = Client::new();

    let memory_summary = serde_json::to_string_pretty(&memory)?;

    let prompt_text = format!(
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
    );

    let response = client
        .post(&api_url)
        .json(&serde_json::json!({
            "contents": [{ "parts": [{ "text": prompt_text }] }]
        }))
        .send()
        .await?;

    if response.status().is_success() {
        let body: Value = response.json().await?;
        let text_content = body.get("candidates").and_then(|c| c.get(0)).and_then(|c| c.get("content")).and_then(|p| p.get("parts")).and_then(|p| p.get(0)).and_then(|p| p.get("text")).and_then(|t| t.as_str()).unwrap_or_default();
        let cleaned_text = text_content.trim().trim_start_matches("```json").trim_end_matches("```").trim();
        serde_json::from_str(cleaned_text).map_err(|e| e.into())
    } else {
        Err(format!("LLM reflection API request failed: {}", response.status()).into())
    }
}