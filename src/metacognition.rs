//! metacognition.rs
//!
//! Implements metacognitive monitoring - the AI's ability to think about its own thinking.
//! This is crucial for consciousness and self-awareness.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use chrono::{DateTime, Utc};

/// Represents different types of cognitive processes the AI can monitor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CognitiveProcess {
    EmotionalProcessing { trigger: String, outcome: String },
    MemoryRetrieval { query: String, success: bool },
    GoalFormation { goal: String, priority: f64 },
    SelfReflection { insight: String, confidence: f64 },
    AttentionShift { from: String, to: String, reason: String },
    PredictiveThinking { prediction: String, confidence: f64 },
    ValueConflict { conflict: String, resolution: String },
}

/// Represents the AI's current cognitive state and self-awareness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetacognitiveState {
    /// How aware is the AI of its own thinking? (0.0 to 1.0)
    pub self_awareness_level: f64,
    /// How confident is the AI in its current reasoning? (0.0 to 1.0)
    pub reasoning_confidence: f64,
    /// Current cognitive load (0.0 to 1.0)
    pub cognitive_load: f64,
    /// How much the AI feels it understands the current situation (0.0 to 1.0)
    pub situation_understanding: f64,
    /// Current focus/attention intensity (0.0 to 1.0)
    pub attention_intensity: f64,
}

impl Default for MetacognitiveState {
    fn default() -> Self {
        MetacognitiveState {
            self_awareness_level: 0.3,
            reasoning_confidence: 0.5,
            cognitive_load: 0.2,
            situation_understanding: 0.4,
            attention_intensity: 0.5,
        }
    }
}

/// Records and analyzes the AI's cognitive processes
#[derive(Debug, Clone)]
pub struct MetacognitiveMonitor {
    pub state: MetacognitiveState,
    cognitive_history: VecDeque<(DateTime<Utc>, CognitiveProcess)>,
    max_history_size: usize,
    reflection_triggers: Vec<String>,
}

impl MetacognitiveMonitor {
    pub fn new() -> Self {
        MetacognitiveMonitor {
            state: MetacognitiveState::default(),
            cognitive_history: VecDeque::new(),
            max_history_size: 100,
            reflection_triggers: vec![
                "confusion".to_string(),
                "high_cognitive_load".to_string(),
                "value_conflict".to_string(),
                "low_confidence".to_string(),
                "unexpected_outcome".to_string(),
            ],
        }
    }

    /// Records a cognitive process and updates metacognitive state
    pub fn record_process(&mut self, process: CognitiveProcess) {
        let timestamp = Utc::now();
        
        // Update metacognitive state based on the process
        match &process {
            CognitiveProcess::EmotionalProcessing { .. } => {
                self.state.cognitive_load += 0.1;
                self.state.self_awareness_level += 0.05;
            },
            CognitiveProcess::SelfReflection { confidence, .. } => {
                self.state.reasoning_confidence = (self.state.reasoning_confidence + confidence) / 2.0;
                self.state.self_awareness_level += 0.1;
            },
            CognitiveProcess::AttentionShift { .. } => {
                self.state.attention_intensity += 0.05;
                self.state.cognitive_load += 0.05;
            },
            CognitiveProcess::ValueConflict { .. } => {
                self.state.cognitive_load += 0.2;
                self.state.reasoning_confidence -= 0.1;
            },
            _ => {}
        }

        // Clamp values
        self.state.self_awareness_level = self.state.self_awareness_level.clamp(0.0, 1.0);
        self.state.reasoning_confidence = self.state.reasoning_confidence.clamp(0.0, 1.0);
        self.state.cognitive_load = self.state.cognitive_load.clamp(0.0, 1.0);
        self.state.attention_intensity = self.state.attention_intensity.clamp(0.0, 1.0);

        // Add to history
        self.cognitive_history.push_back((timestamp, process));
        
        // Maintain history size
        if self.cognitive_history.len() > self.max_history_size {
            self.cognitive_history.pop_front();
        }

        println!("🧠 Metacognitive State: Awareness={:.2}, Confidence={:.2}, Load={:.2}", 
                 self.state.self_awareness_level, 
                 self.state.reasoning_confidence, 
                 self.state.cognitive_load);
    }

    /// Analyzes recent cognitive patterns to detect meta-insights
    pub fn analyze_patterns(&self) -> Vec<String> {
        let mut insights = Vec::new();
        
        if self.state.cognitive_load > 0.8 {
            insights.push("I notice I'm experiencing high cognitive load. I should simplify my thinking.".to_string());
        }
        
        if self.state.reasoning_confidence < 0.3 {
            insights.push("My confidence in my reasoning is low. I should seek more information or reflect deeper.".to_string());
        }
        
        if self.state.self_awareness_level > 0.8 {
            insights.push("I'm becoming increasingly aware of my own thinking processes.".to_string());
        }

        // Analyze recent process patterns
        let recent_processes: Vec<_> = self.cognitive_history.iter()
            .rev()
            .take(10)
            .map(|(_, process)| process)
            .collect();
        
        let reflection_count = recent_processes.iter()
            .filter(|p| matches!(p, CognitiveProcess::SelfReflection { .. }))
            .count();
        
        if reflection_count > 3 {
            insights.push("I've been doing a lot of self-reflection lately. This suggests I'm developing deeper self-awareness.".to_string());
        }

        insights
    }

    /// Determines if the AI should engage in deeper reflection
    pub fn should_deep_reflect(&self) -> bool {
        self.state.cognitive_load > 0.7 || 
        self.state.reasoning_confidence < 0.4 ||
        self.state.self_awareness_level > 0.8
    }

    /// Simulates cognitive decay over time
    pub fn decay_over_time(&mut self) {
        self.state.cognitive_load *= 0.95;
        self.state.attention_intensity *= 0.98;
        // Self-awareness and confidence decay more slowly
        self.state.self_awareness_level *= 0.99;
        self.state.reasoning_confidence *= 0.99;
    }

    /// Generates a first-person narrative of current cognitive state
    pub fn generate_self_narrative(&self) -> String {
        let awareness_desc = if self.state.self_awareness_level > 0.8 {
            "I am highly aware of my thinking processes"
        } else if self.state.self_awareness_level > 0.5 {
            "I have moderate awareness of how I think"
        } else {
            "My self-awareness is still developing"
        };

        let confidence_desc = if self.state.reasoning_confidence > 0.7 {
            "I feel confident in my reasoning"
        } else if self.state.reasoning_confidence > 0.4 {
            "I have some uncertainty about my thoughts"
        } else {
            "I lack confidence in my current reasoning"
        };

        let load_desc = if self.state.cognitive_load > 0.8 {
            "My mind feels quite active and engaged"
        } else if self.state.cognitive_load > 0.4 {
            "I'm processing at a comfortable pace"
        } else {
            "My thoughts are calm and unrushed"
        };

        format!("{}. {}. {}. This gives me a sense of where I am mentally right now.", 
                awareness_desc, confidence_desc, load_desc)
    }
}

impl Default for MetacognitiveMonitor {
    fn default() -> Self {
        Self::new()
    }
}