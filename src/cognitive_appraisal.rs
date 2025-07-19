//! cognitive_appraisal.rs
//!
//! Defines a flexible structure for appraised emotions and provides
//! functionality for appraising emotions from text.

use serde::Deserialize;
use crate::{llm_api, memory::Memory};

/// **NEW**: A flexible structure to hold any appraised emotion from the LLM.
/// The `OccEmotion` enum is no longer used for deserialization.
#[derive(Debug, Clone, Deserialize)]
pub struct AppraisedEmotion {
    /// The name of the emotion, as identified by the LLM (e.g., "Joy", "Apprehension", "Nostalgia").
    pub emotion: String,
    /// The mapped VADN coordinates for this emotion.
    pub vadn: AffectiveStateChange,
    /// Any additional details the LLM provides.
    pub details: serde_json::Value,
}

/// **NEW**: Represents the direct VADN change proposed by the LLM.
#[derive(Debug, Clone, Deserialize, Copy)]
pub struct AffectiveStateChange {
    pub valence: f64,
    pub arousal: f64,
    pub dominance: f64,
    pub novelty: f64,
}


/// Appraises the emotion from a user's prompt by calling the LLM.
pub async fn appraise_emotion_from_prompt(user_prompt: &str, memory: &Memory) -> Result<AppraisedEmotion, String> {
    match llm_api::call_llm_for_appraisal(user_prompt, memory).await {
        Ok(emotion) => Ok(emotion),
        Err(e) => {
            let err_msg = format!("🔥 Appraisal Error: {}. Falling back to Neutral.", e);
            eprintln!("{}", err_msg);
            Err(err_msg)
        }
    }
}