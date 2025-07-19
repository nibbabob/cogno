//! cognitive_appraisal.rs
//!
//! Defines emotions based on the OCC (Ortony, Clore, Collins) model.
//! These are the structured emotional categories we'll ask the LLM to identify.

use serde::Deserialize;

/// Represents discrete emotions from the OCC model, deserializable from LLM output.
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