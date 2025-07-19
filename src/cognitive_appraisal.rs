//! cognitive_appraisal.rs
//!
//! Defines emotions based on the OCC (Ortony, Clore, Collins) model
//! and provides functionality for appraising emotions from text.

use serde::Deserialize;
use crate::llm_api;

// ... (No changes to the OccEmotion enum)
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "emotion", content = "details", rename_all = "PascalCase")]
pub enum OccEmotion {
    // High positive valence emotions
    Joy {
        intensity: f64,
        focus: String,
    },
    Pride {
        intensity: f64,
        #[serde(default)]
        target: String,
    },
    Gratitude {
        intensity: f64,
        #[serde(default)]
        agent: String,
    },
    Satisfaction {
        intensity: f64,
        #[serde(default)]
        goal: String,
    },
    Relief {
        intensity: f64,
    },

    // Negative valence emotions
    Distress {
        intensity: f64,
        focus: String,
    },
    Fear {
        likelihood: f64,
        #[serde(default)]
        event: String,
    },
    Anger {
        intensity: f64,
        #[serde(default)]
        target: String,
    },
    Shame {
        intensity: f64,
        #[serde(default)]
        action: String,
    },
    Disappointment {
        intensity: f64,
        #[serde(default)]
        expectation: String,
    },

    // Prospect-based emotions
    Hope {
        likelihood: f64,
        #[serde(default)]
        event: String,
    },

    // Neutral or unhandled
    Neutral,
}

// Default implementation for unhandled cases
impl Default for OccEmotion {
    fn default() -> Self {
        OccEmotion::Neutral
    }
}


/// Appraises the emotion from a user's prompt by calling the LLM.
pub async fn appraise_emotion_from_prompt(user_prompt: &str) -> OccEmotion {
    // FIX: Instead of unwrap_or_default(), we now handle the result properly.
    match llm_api::call_llm_for_appraisal(user_prompt).await {
        Ok(emotion) => emotion,
        Err(e) => {
            // This will print a detailed error if the API call or parsing fails.
            eprintln!("🔥 Appraisal Error: {}. Falling back to Neutral.", e);
            OccEmotion::default()
        }
    }
}