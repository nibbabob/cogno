//! Core Module
//!
//! Manages the underlying emotional state and self-reflection.

use crate::cognitive_appraisal::{AppraisedEmotion, AffectiveStateChange};
use crate::llm_api;
use crate::memory::Memory;

#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize)]
pub struct AffectiveState {
    pub valence: f64,
    pub arousal: f64,
    pub dominance: f64,
    pub novelty: f64,
}

// ... (AffectiveState impl is unchanged) ...
impl AffectiveState {
    pub fn new_neutral() -> Self {
        AffectiveState {
            valence: 0.0,
            arousal: 0.3,
            dominance: 0.1,
            novelty: 0.0,
        }
    }

    /// Internal method to apply changes and clamp values
    fn apply_change(&mut self, change: AffectiveStateChange) { // UPDATED to take AffectiveStateChange
        self.valence = (self.valence + change.valence).clamp(-1.0, 1.0);
        self.arousal = (self.arousal + change.arousal).clamp(0.0, 1.0);
        self.dominance = (self.dominance + change.dominance).clamp(-1.0, 1.0);
        self.novelty = (self.novelty + change.novelty).clamp(-1.0, 1.0);
    }
    
    fn decay(&mut self, baseline: AffectiveState, rate: f64) {
        let rate = rate.clamp(0.0, 1.0);
        self.valence += (baseline.valence - self.valence) * rate;
        self.arousal += (baseline.arousal - self.arousal) * rate;
        self.dominance += (baseline.dominance - self.dominance) * rate;
        self.novelty += (baseline.novelty - self.novelty) * rate;
    }
}


pub struct AffectiveCore {
    current_state: AffectiveState,
    pub memory: Memory,
    decay_rate: f64,
    empathy_factor: f64,
}

impl AffectiveCore {
    /// Creates a new AffectiveCore, initializing state from its memory's personality.
    pub fn new() -> Self {
        let memory = Memory::new();
        AffectiveCore {
            current_state: memory.personality.baseline_state,
            memory,
            decay_rate: 0.15,
            empathy_factor: 0.8,
        }
    }

    // --- ADD THIS METHOD BACK ---
    /// Returns a copy of the current affective state.
    pub fn current_state(&self) -> AffectiveState {
        self.current_state
    }
    
    /// Processes an appraised emotion, updating the internal state.
    pub fn process_emotion(&mut self, emotion: &AppraisedEmotion) {
        let change = emotion.vadn;
        let blended_change = AffectiveStateChange {
            valence: change.valence * self.empathy_factor,
            arousal: change.arousal * self.empathy_factor,
            dominance: change.dominance * self.empathy_factor,
            novelty: change.novelty * self.empathy_factor,
        };
        self.current_state.apply_change(blended_change);

        let full_emotion_details = format!(
            "Emotion: '{}', VADN: {:?}, Details: {}",
            emotion.emotion,
            emotion.vadn,
            emotion.details.to_string()
        );
        
        if emotion.vadn.valence.abs() > 0.6 || emotion.vadn.arousal > 0.7 {
            self.memory.record_milestone(full_emotion_details);
        }
    }

    /// Applies emotional regulation, decaying the state toward its personality's baseline.
    pub fn regulate_emotion(&mut self) {
        let baseline = self.memory.personality.baseline_state;
        self.current_state.decay(baseline, self.decay_rate);
    }
    
    /// Triggers the self-reflection process.
    pub async fn reflect(&mut self) {
        println!("\n--- SELF-REFLECTION TRIGGERED ---");
        match llm_api::call_llm_for_reflection(&self.memory).await {
            Ok(new_personality) => {
                println!("💡 Reflection successful. Personality has been updated.");
                println!("Old personality: {:?}", self.memory.personality);
                println!("New personality: {:?}", new_personality);
                self.memory.personality = new_personality;
            }
            Err(e) => {
                eprintln!("🔥 Reflection Error: {}", e);
            }
        }
        println!("--- SELF-REFLECTION COMPLETE ---\n");
    }

    // ... (rest of the file is unchanged) ...
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
    
    fn synthesize_feeling(&self, v: f64, a: f64, d: f64) -> String {
        if v > 0.4 && a > 0.45 { "elated and proud".to_string() }
        else if v > 0.4 { "pleased and content".to_string() }
        else if v < -0.5 && a > 0.5 {
            if d > 0.4 { "indignant and assertive".to_string() }
            else { "anxious and distressed".to_string() }
        }
        else if v < -0.5 {
            if d < -0.4 { "dejected and powerless".to_string() }
            else { "somber and disappointed".to_string() }
        }
        else if a > 0.6 { "alert and focused".to_string() }
        else if a < 0.25 { "calm and relaxed".to_string() }
        else { "calmly neutral".to_string() }
    }
}

// ... (Default impl and helper functions are unchanged) ...
impl Default for AffectiveCore {
    fn default() -> Self {
        AffectiveCore::new()
    }
}

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