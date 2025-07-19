//! Core Module
//! 
//! Manages the underlying emotional state using dimensional models of affect.
//! Based on research in affective computing and psychological theories of emotion.

/// Core dimensions of emotional experience based on circumplex models
#[derive(Debug, Clone, Copy, Default)]
pub struct AffectiveState {
    /// Pleasure-displeasure: how positive/negative the experience is (-1.0 to 1.0)
    pub valence: f64,
    /// Activation-deactivation: how energized/calm the experience is (0.0 to 1.0)  
    pub arousal: f64,
    /// Control-submission: how much agency/power is felt (-1.0 to 1.0)
    pub dominance: f64,
    /// Familiarity-novelty: how expected/surprising the situation is (-1.0 to 1.0)
    pub novelty: f64,
}

impl AffectiveState {
    pub fn new_neutral() -> Self {
        AffectiveState {
            valence: 0.0,
            arousal: 0.3,    // Slightly alert
            dominance: 0.1,  // Slightly confident
            novelty: 0.0,    // Neutral familiarity
        }
    }
    
    /// Apply changes while maintaining realistic bounds
    pub fn apply_change(&mut self, change: AffectiveState) {
        self.valence = (self.valence + change.valence).clamp(-1.0, 1.0);
        self.arousal = (self.arousal + change.arousal).clamp(0.0, 1.0);
        self.dominance = (self.dominance + change.dominance).clamp(-1.0, 1.0);
        self.novelty = (self.novelty + change.novelty).clamp(-1.0, 1.0);
    }
    
    /// Gradual return to baseline (emotional regulation)
    pub fn decay_toward_baseline(&mut self, baseline: AffectiveState, decay_rate: f64) {
        let rate = decay_rate.clamp(0.0, 1.0);
        self.valence += (baseline.valence - self.valence) * rate;
        self.arousal += (baseline.arousal - self.arousal) * rate;
        self.dominance += (baseline.dominance - self.dominance) * rate;
        self.novelty += (baseline.novelty - self.novelty) * rate;
    }
}

pub struct AffectiveCore {
    pub current_state: AffectiveState,
    pub baseline_state: AffectiveState,
    pub emotional_history: Vec<String>, // Track recent emotions
}

impl AffectiveCore {
    pub fn new() -> Self {
        let baseline = AffectiveState::new_neutral();
        AffectiveCore {
            current_state: baseline,
            baseline_state: baseline,
            emotional_history: Vec::new(),
        }
    }
    
    /// Map OCC emotions to affective changes based on psychological research
    pub fn emotion_to_affective_change(&self, emotion: &crate::cognitive_appraisal::OccEmotion) -> AffectiveState {
        use crate::cognitive_appraisal::OccEmotion::*;
        
        match emotion {
            // High positive valence emotions
            Joy { intensity, .. } => AffectiveState {
                valence: 0.6 * intensity,
                arousal: 0.4 * intensity,
                dominance: 0.2 * intensity,
                novelty: 0.0,
            },
            
            Pride { intensity, .. } => AffectiveState {
                valence: 0.7 * intensity,
                arousal: 0.3 * intensity,
                dominance: 0.8 * intensity, // High sense of control
                novelty: -0.1,
            },
            
            Gratitude { .. } => AffectiveState {
                valence: 0.8,
                arousal: 0.3,
                dominance: -0.2, // Feeling indebted/humble
                novelty: 0.1,
            },
            
            Satisfaction { .. } => AffectiveState {
                valence: 0.6,
                arousal: -0.2, // Calming
                dominance: 0.4,
                novelty: -0.3, // Expected outcome
            },
            
            Relief { .. } => AffectiveState {
                valence: 0.4,
                arousal: -0.5, // Large arousal reduction
                dominance: 0.2,
                novelty: 0.0,
            },
            
            // Negative valence emotions
            Distress { intensity, .. } => AffectiveState {
                valence: -0.6 * intensity,
                arousal: 0.4 * intensity,
                dominance: -0.3 * intensity,
                novelty: 0.1,
            },
            
            Fear { likelihood, .. } => AffectiveState {
                valence: -0.7 * likelihood,
                arousal: 0.8 * likelihood, // High activation
                dominance: -0.8 * likelihood, // Low control
                novelty: 0.3,
            },
            
            Anger { .. } => AffectiveState {
                valence: -0.6,
                arousal: 0.8, // High energy
                dominance: 0.6, // Feeling powerful/aggressive
                novelty: 0.2,
            },
            
            Shame { intensity, .. } => AffectiveState {
                valence: -0.8 * intensity,
                arousal: 0.5 * intensity,
                dominance: -0.9 * intensity, // Very low control/power
                novelty: 0.0,
            },
            
            Disappointment { .. } => AffectiveState {
                valence: -0.5,
                arousal: -0.3, // Deflating
                dominance: -0.4,
                novelty: -0.2, // Violated expectation
            },
            
            // Prospect emotions
            Hope { likelihood, .. } => AffectiveState {
                valence: 0.4 * likelihood,
                arousal: 0.3 * likelihood,
                dominance: 0.1,
                novelty: 0.2,
            },
            
            // Default for any unhandled emotions
            _ => AffectiveState::default(),
        }
    }
    
    pub fn process_emotion(&mut self, emotion: &crate::cognitive_appraisal::OccEmotion) {
        let change = self.emotion_to_affective_change(emotion);
        self.current_state.apply_change(change);
        
        // Track emotional history
        let emotion_name = format!("{:?}", emotion).split('{').next().unwrap_or("Unknown").to_string();
        self.emotional_history.push(emotion_name);
        if self.emotional_history.len() > 10 {
            self.emotional_history.remove(0);
        }
    }
    
    pub fn regulate_emotion(&mut self) {
        // Simulate natural emotional regulation (return to baseline over time)
        self.current_state.decay_toward_baseline(self.baseline_state, 0.05);
    }
}