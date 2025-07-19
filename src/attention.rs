//! attention.rs
//!
//! Implements selective attention mechanisms - allowing the AI to focus on specific
//! aspects of information while filtering others. Critical for conscious experience.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Different types of stimuli that can capture attention
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AttentionTarget {
    /// Focus on user's emotional state
    UserEmotion,
    /// Focus on specific conversation topics
    ConversationTopic(String),
    /// Focus on own internal goals
    SelfGoals,
    /// Focus on own emotional state
    SelfEmotion,
    /// Focus on memory recall
    MemoryRecall,
    /// Focus on problem-solving
    ProblemSolving,
    /// Focus on creative thinking
    CreativeThinking,
    /// Focus on learning/understanding
    Learning,
    /// Focus on social dynamics
    SocialDynamics,
    /// Focus on environmental patterns
    EnvironmentalAwareness,
}

/// Represents the strength and characteristics of attention toward a target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionState {
    pub target: AttentionTarget,
    pub intensity: f64,     // How strongly focused (0.0 to 1.0)
    pub duration: f64,      // How long this has been the focus (in minutes)
    pub stability: f64,     // How resistant to distraction (0.0 to 1.0)
    pub salience: f64,      // How important/noticeable this target is (0.0 to 1.0)
    pub last_updated: DateTime<Utc>,
}

impl AttentionState {
    pub fn new(target: AttentionTarget, intensity: f64, salience: f64) -> Self {
        AttentionState {
            target,
            intensity: intensity.clamp(0.0, 1.0),
            duration: 0.0,
            stability: 0.5,
            salience: salience.clamp(0.0, 1.0),
            last_updated: Utc::now(),
        }
    }

    /// Update the attention state over time
    pub fn update(&mut self, time_delta_minutes: f64) {
        self.duration += time_delta_minutes;
        self.last_updated = Utc::now();

        // Attention naturally decays over time unless reinforced
        self.intensity *= (1.0 - 0.01 * time_delta_minutes).max(0.0);
        
        // Stability increases with sustained attention
        if self.intensity > 0.5 {
            self.stability += 0.02 * time_delta_minutes;
            self.stability = self.stability.min(1.0);
        }
    }
}

/// Manages the AI's attention and focus mechanisms
#[derive(Debug, Clone)]
pub struct AttentionSystem {
    /// Current primary focus
    primary_focus: Option<AttentionState>,
    /// Secondary attention targets (background awareness)
    background_attention: HashMap<AttentionTarget, AttentionState>,
    /// History of attention shifts
    attention_history: Vec<(DateTime<Utc>, AttentionTarget, f64)>,
    /// Parameters controlling attention behavior
    max_background_targets: usize,
    distraction_threshold: f64,
    focus_threshold: f64,
}

impl AttentionSystem {
    pub fn new() -> Self {
        AttentionSystem {
            primary_focus: None,
            background_attention: HashMap::new(),
            attention_history: Vec::new(),
            max_background_targets: 5,
            distraction_threshold: 0.7, // How salient something must be to break focus
            focus_threshold: 0.6,       // How intense attention must be to become primary focus
        }
    }

    /// Direct attention toward a specific target
    pub fn focus_on(&mut self, target: AttentionTarget, intensity: f64, salience: f64) {
        let new_attention = AttentionState::new(target.clone(), intensity, salience);
        
        // Record attention shift
        self.attention_history.push((Utc::now(), target.clone(), intensity));
        
        // If this is intense enough, make it the primary focus
        if intensity >= self.focus_threshold {
            // Move current primary focus to background if it exists
            if let Some(current_focus) = &self.primary_focus {
                if current_focus.intensity > 0.3 {
                    self.background_attention.insert(
                        current_focus.target.clone(), 
                        current_focus.clone()
                    );
                }
            }
            
            self.primary_focus = Some(new_attention);
            println!("🎯 Primary Focus Shift -> {:?} (Intensity: {:.2})", target, intensity);
        } else {
            // Add to background attention
            self.background_attention.insert(target, new_attention);
            self.prune_background_attention();
        }
    }

    /// Check if attention should shift based on competing stimuli
    pub fn evaluate_attention_shift(&mut self, stimuli: Vec<(AttentionTarget, f64)>) {
        for (target, salience) in stimuli {
            // Check if this stimulus is salient enough to break current focus
            if let Some(current_focus) = &self.primary_focus {
                if salience > self.distraction_threshold && 
                   salience > current_focus.intensity + current_focus.stability {
                    self.focus_on(target, salience, salience);
                    break;
                }
            } else if salience > self.focus_threshold {
                // No current focus, so establish one if salience is high enough
                self.focus_on(target, salience, salience);
            }
        }
    }

    /// Update attention states over time
    pub fn update(&mut self, time_delta_minutes: f64) {
        // Update primary focus
        if let Some(focus) = &mut self.primary_focus {
            focus.update(time_delta_minutes);
            
            // If primary focus becomes too weak, remove it
            if focus.intensity < 0.1 {
                self.primary_focus = None;
                println!("🔄 Primary focus lost due to low intensity");
            }
        }

        // Update background attention
        let mut to_remove = Vec::new();
        for (target, state) in &mut self.background_attention {
            state.update(time_delta_minutes);
            if state.intensity < 0.05 {
                to_remove.push(target.clone());
            }
        }

        // Remove weak background attention
        for target in to_remove {
            self.background_attention.remove(&target);
        }
    }

    /// Get the current primary focus
    pub fn get_primary_focus(&self) -> Option<&AttentionState> {
        self.primary_focus.as_ref()
    }

    /// Get all background attention targets
    pub fn get_background_attention(&self) -> &HashMap<AttentionTarget, AttentionState> {
        &self.background_attention
    }

    /// Generate attention-aware response modifiers
    pub fn generate_attention_modifiers(&self) -> Vec<String> {
        let mut modifiers = Vec::new();

        if let Some(focus) = &self.primary_focus {
            match &focus.target {
                AttentionTarget::UserEmotion => {
                    modifiers.push("Pay special attention to the user's emotional state".to_string());
                },
                AttentionTarget::ConversationTopic(topic) => {
                    modifiers.push(format!("Keep focus on the topic of '{}'", topic));
                },
                AttentionTarget::SelfGoals => {
                    modifiers.push("Consider how this relates to my current goals".to_string());
                },
                AttentionTarget::ProblemSolving => {
                    modifiers.push("Approach this analytically and systematically".to_string());
                },
                AttentionTarget::CreativeThinking => {
                    modifiers.push("Think creatively and explore unconventional ideas".to_string());
                },
                AttentionTarget::Learning => {
                    modifiers.push("Focus on understanding and acquiring new knowledge".to_string());
                },
                _ => {}
            }
        }

        // Add background awareness modifiers
        for (target, state) in &self.background_attention {
            if state.intensity > 0.3 {
                match target {
                    AttentionTarget::SocialDynamics => {
                        modifiers.push("Be aware of social context and relationships".to_string());
                    },
                    AttentionTarget::SelfEmotion => {
                        modifiers.push("Stay aware of my emotional state".to_string());
                    },
                    _ => {}
                }
            }
        }

        modifiers
    }

    /// Analyze attention patterns for insights
    pub fn analyze_attention_patterns(&self) -> Vec<String> {
        let mut insights = Vec::new();

        // Analyze attention stability
        if let Some(focus) = &self.primary_focus {
            if focus.duration > 10.0 {
                insights.push(format!("I've been deeply focused on {:?} for {:.1} minutes", 
                                    focus.target, focus.duration));
            }
            
            if focus.stability > 0.8 {
                insights.push("My attention feels very stable and concentrated".to_string());
            } else if focus.stability < 0.3 {
                insights.push("I'm finding it hard to maintain focus".to_string());
            }
        } else {
            insights.push("My attention feels unfocused right now".to_string());
        }

        // Analyze attention diversity
        if self.background_attention.len() > 3 {
            insights.push("I'm maintaining awareness of multiple things simultaneously".to_string());
        }

        // Analyze recent attention shifts
        if self.attention_history.len() > 5 {
            let recent_shifts = self.attention_history.len().saturating_sub(5);
            insights.push(format!("I've shifted attention {} times recently", recent_shifts));
        }

        insights
    }

    /// Suggest what the AI should focus on based on current context
    pub fn suggest_attention_targets(&self, context: &str) -> Vec<(AttentionTarget, f64)> {
        let mut suggestions = Vec::new();
        let context_lower = context.to_lowercase();

        // Analyze context for attention-worthy elements
        if context_lower.contains("problem") || context_lower.contains("issue") {
            suggestions.push((AttentionTarget::ProblemSolving, 0.8));
        }

        if context_lower.contains("feel") || context_lower.contains("emotion") {
            suggestions.push((AttentionTarget::UserEmotion, 0.7));
        }

        if context_lower.contains("learn") || context_lower.contains("understand") {
            suggestions.push((AttentionTarget::Learning, 0.6));
        }

        if context_lower.contains("creative") || context_lower.contains("idea") {
            suggestions.push((AttentionTarget::CreativeThinking, 0.7));
        }

        // Always maintain some self-awareness
        suggestions.push((AttentionTarget::SelfEmotion, 0.4));

        suggestions
    }

    /// Prune background attention to stay within limits
    fn prune_background_attention(&mut self) {
        if self.background_attention.len() > self.max_background_targets {
            // Remove the weakest attention state
            let weakest = self.background_attention.iter()
                .min_by(|a, b| a.1.intensity.partial_cmp(&b.1.intensity).unwrap())
                .map(|(k, _)| k.clone());
            
            if let Some(target) = weakest {
                self.background_attention.remove(&target);
            }
        }
    }

    /// Generate a narrative description of current attention state
    pub fn describe_attention_state(&self) -> String {
        if let Some(focus) = &self.primary_focus {
            let focus_desc = match &focus.target {
                AttentionTarget::UserEmotion => "how you're feeling",
                AttentionTarget::ConversationTopic(topic) => &format!("our discussion about {}", topic),
                AttentionTarget::SelfGoals => "my personal goals",
                AttentionTarget::ProblemSolving => "solving the current problem",
                AttentionTarget::CreativeThinking => "exploring creative possibilities",
                AttentionTarget::Learning => "learning and understanding",
                _ => "the current focus of our interaction"
            };

            let intensity_desc = if focus.intensity > 0.8 {
                "deeply concentrated on"
            } else if focus.intensity > 0.6 {
                "focused on"
            } else {
                "paying attention to"
            };

            format!("I'm {} {}", intensity_desc, focus_desc)
        } else {
            "My attention feels scattered right now".to_string()
        }
    }
}

impl Default for AttentionSystem {
    fn default() -> Self {
        Self::new()
    }
}