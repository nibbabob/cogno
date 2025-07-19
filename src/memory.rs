//! src/memory.rs
//!
//! Manages long-term memory, user profile, and the AI's own personality.

use crate::core::AffectiveState; // Import AffectiveState
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **NEW**: Defines the core, long-term personality of the AI.
/// This is the baseline that the AI will decay towards.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Personality {
    pub baseline_state: AffectiveState,
}

impl Default for Personality {
    fn default() -> Self {
        Personality {
            baseline_state: AffectiveState::new_neutral(),
        }
    }
}

/// Stores key information about the user.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserProfile {
    pub name: Option<String>,
    pub preferences: HashMap<String, String>,
}

/// Represents the AI's memory, now including its own personality.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub user_profile: UserProfile,
    pub interaction_count: u64,
    pub emotional_milestones: Vec<String>,
    pub personality: Personality, // ADD THIS
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            user_profile: UserProfile::default(),
            interaction_count: 0,
            emotional_milestones: Vec::new(),
            personality: Personality::default(), // AND THIS
        }
    }

    /// A simple method to update the user's name if found in a prompt.
    pub fn learn_from_prompt(&mut self, prompt: &str) {
        let lower_prompt = prompt.to_lowercase();
        if self.user_profile.name.is_none() { // Only learn if not already known
            if let Some(index) = lower_prompt.find("my name is") {
                let name_part = &prompt[index + "my name is".len()..];
                if let Some(name) = name_part.trim().split([' ', ',', '.']).next() {
                    if !name.is_empty() {
                        let first_char = name.chars().next().unwrap().to_uppercase().to_string();
                        self.user_profile.name = Some(format!("{}{}", first_char, &name[1..]));
                    }
                }
            }
        }
    }

    /// Records a significant emotional event.
    pub fn record_milestone(&mut self, emotion_details: String) {
        self.emotional_milestones.push(emotion_details);
        // Keep the list from growing too large
        if self.emotional_milestones.len() > 20 {
            self.emotional_milestones.remove(0);
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}