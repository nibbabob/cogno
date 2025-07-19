//! Core Module
//!
//! Manages the underlying emotional state using dimensional models of affect.
//! Based on research in affective computing and psychological theories of emotion.

// No changes needed for AffectiveState
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
            arousal: 0.3,   // Slightly alert
            dominance: 0.1,  // Slightly confident
            novelty: 0.0,    // Neutral familiarity
        }
    }

    /// Internal method to apply changes and clamp values
    fn apply_change(&mut self, change: AffectiveState) {
        self.valence = (self.valence + change.valence).clamp(-1.0, 1.0);
        self.arousal = (self.arousal + change.arousal).clamp(0.0, 1.0);
        self.dominance = (self.dominance + change.dominance).clamp(-1.0, 1.0);
        self.novelty = (self.novelty + change.novelty).clamp(-1.0, 1.0);
    }
    
    /// Internal method for state decay
    fn decay(&mut self, baseline: AffectiveState, rate: f64) {
        let rate = rate.clamp(0.0, 1.0);
        self.valence += (baseline.valence - self.valence) * rate;
        self.arousal += (baseline.arousal - self.arousal) * rate;
        self.dominance += (baseline.dominance - self.dominance) * rate;
        self.novelty += (baseline.novelty - self.novelty) * rate;
    }
}

// NEW: Configuration for personality and behavior
#[derive(Debug, Clone, Copy)]
pub struct AffectiveConfig {
    pub baseline_state: AffectiveState,
    pub decay_rate: f64,
    pub empathy_factor: f64, // How much the AI is influenced by the user's emotion (0.0 to 1.0)
}

impl Default for AffectiveConfig {
    fn default() -> Self {
        AffectiveConfig {
            baseline_state: AffectiveState::new_neutral(),
            decay_rate: 0.15, // A slightly faster decay
            empathy_factor: 0.5, // Moderately empathetic
        }
    }
}

// UPDATED: AffectiveCore now uses the config and has private fields
pub struct AffectiveCore {
    current_state: AffectiveState,
    config: AffectiveConfig,
    emotional_history: Vec<String>,
}

impl AffectiveCore {
    /// Creates a new AffectiveCore with a given configuration.
    pub fn new(config: AffectiveConfig) -> Self {
        AffectiveCore {
            current_state: config.baseline_state,
            config,
            emotional_history: Vec::new(),
        }
    }

    // --- Public API ---

    /// Processes an appraised emotion, updating the internal state.
    pub fn process_emotion(&mut self, emotion: &crate::cognitive_appraisal::OccEmotion) {
        // 1. Get the raw emotional change from the stimulus
        let change = self.emotion_to_affective_change(emotion);

        // 2. Apply empathy model: blend the change with the current state
        let empathy = self.config.empathy_factor;
        let blended_change = AffectiveState {
            valence: change.valence * empathy,
            arousal: change.arousal * empathy,
            dominance: change.dominance * empathy,
            novelty: change.novelty * empathy,
        };
        self.current_state.apply_change(blended_change);

        // 3. Track history
        let full_emotion_details = format!("{:?}", emotion);
        self.emotional_history.push(full_emotion_details);
        if self.emotional_history.len() > 10 {
            self.emotional_history.remove(0);
        }
    }

    /// Applies emotional regulation, decaying the state toward its baseline.
    pub fn regulate_emotion(&mut self) {
        self.current_state.decay(self.config.baseline_state, self.config.decay_rate);
    }

    /// **NEW**: Generates the text for the VADN-Aware Instructional Microprompt.
    pub fn get_instructional_prompt_text(&self) -> String {
        let v = self.current_state.valence;
        let a = self.current_state.arousal;
        let d = self.current_state.dominance;
        let n = self.current_state.novelty;
        
        // Synthesize a summary of the feeling
        let summary = self.synthesize_feeling(v, a, d);

        format!(
            "Your current internal affective state is described by these dimensions:\n\
            - Valence (Pleasure): {} ({:.2})\n\
            - Arousal (Energy): {} ({:.2})\n\
            - Dominance (Control): {} ({:.2})\n\
            - Novelty (Surprise): {} ({:.2})\n\
            \n\
            Overall, this makes you feel {}. Subtly reflect this state in your response.",
            describe_valence(v), v,
            describe_arousal(a), a,
            describe_dominance(d), d,
            describe_novelty(n), n,
            summary
        )
    }

    // --- Getters for state observation ---
    pub fn current_state(&self) -> AffectiveState { self.current_state }
    pub fn history(&self) -> &Vec<String> { &self.emotional_history }

    // --- Internal Logic ---

    /// Maps discrete emotions to VADN state changes. (Unchanged from your version)
    fn emotion_to_affective_change(&self, emotion: &crate::cognitive_appraisal::OccEmotion) -> AffectiveState {
        use crate::cognitive_appraisal::OccEmotion::*;
        match emotion {
            Joy { intensity, .. } => AffectiveState { valence: 0.6 * intensity, arousal: 0.4 * intensity, dominance: 0.2 * intensity, novelty: 0.1 * intensity },
            Pride { intensity, .. } => AffectiveState { valence: 0.7 * intensity, arousal: 0.3 * intensity, dominance: 0.8 * intensity, novelty: -0.1 },
            Gratitude { intensity, .. } => AffectiveState { valence: 0.8 * intensity, arousal: 0.2 * intensity, dominance: -0.3 * intensity, novelty: 0.2 },
            Satisfaction { .. } => AffectiveState { valence: 0.6, arousal: -0.2, dominance: 0.4, novelty: -0.3 },
            Relief { .. } => AffectiveState { valence: 0.5, arousal: -0.5, dominance: 0.2, novelty: 0.0 },
            Distress { intensity, .. } => AffectiveState { valence: -0.6 * intensity, arousal: 0.4 * intensity, dominance: -0.3 * intensity, novelty: 0.1 },
            Fear { likelihood, .. } => AffectiveState { valence: -0.7 * likelihood, arousal: 0.8 * likelihood, dominance: -0.8 * likelihood, novelty: 0.6 * likelihood },
            Anger { intensity, .. } => AffectiveState { valence: -0.6 * intensity, arousal: 0.7 * intensity, dominance: 0.6 * intensity, novelty: 0.2 },
            Shame { intensity, .. } => AffectiveState { valence: -0.8 * intensity, arousal: 0.2 * intensity, dominance: -0.9 * intensity, novelty: 0.0 },
            Disappointment { .. } => AffectiveState { valence: -0.5, arousal: -0.3, dominance: -0.4, novelty: -0.2 },
            Hope { likelihood, .. } => AffectiveState { valence: 0.4 * likelihood, arousal: 0.3 * likelihood, dominance: 0.1, novelty: 0.2 },
            _ => AffectiveState::default(),
        }
    }

    /// Synthesizes a feeling description from the VADN state.
    fn synthesize_feeling(&self, v: f64, a: f64, d: f64) -> String {
        if v > 0.5 && a > 0.5 { "elated and energetic".to_string() }
        else if v > 0.5 { "pleased and content".to_string() }
        else if v < -0.5 && a > 0.6 {
            if d > 0.4 { "indignant and assertive".to_string() }
            else { "anxious and distressed".to_string() }
        }
        else if v < -0.5 {
            if d < -0.4 { "dejected and powerless".to_string() }
            else { "somber and disappointed".to_string() }
        }
        else if a > 0.7 { "highly alert and focused".to_string() }
        else if a < 0.2 { "calm and relaxed".to_string() }
        else { "calmly neutral".to_string() }
    }
}

// Default implementation for easy setup
impl Default for AffectiveCore {
    fn default() -> Self {
        AffectiveCore::new(AffectiveConfig::default())
    }
}

// --- Private Helper Functions for Prompt Generation ---

fn describe_valence(v: f64) -> &'static str {
    if v > 0.7 { "very positive" } else if v > 0.3 { "positive" }
    else if v < -0.7 { "very negative" } else if v < -0.3 { "negative" }
    else { "neutral" }
}

fn describe_arousal(a: f64) -> &'static str {
    if a > 0.8 { "very high energy" } else if a > 0.6 { "high energy" }
    else if a < 0.2 { "very low energy" } else if a < 0.4 { "low energy" }
    else { "moderate energy" }
}

fn describe_dominance(d: f64) -> &'static str {
    if d > 0.7 { "very high control" } else if d > 0.3 { "in control" }
    else if d < -0.7 { "very low control" } else if d < -0.3 { "lacking control" }
    else { "neutral control" }
}

fn describe_novelty(n: f64) -> &'static str {
    if n > 0.7 { "highly surprising" } else if n > 0.3 { "surprising" }
    else if n < -0.7 { "highly expected" } else if n < -0.3 { "expected" }
    else { "neutral" }
}