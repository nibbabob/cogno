//! metacognition.rs
//!
//! Enhanced metacognitive monitoring with complete reflection trigger system

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
    ErrorRecovery { error_type: String, strategy: String },
    CreativeThinking { concept: String, originality: f64 },
    SocialInteraction { context: String, empathy_level: f64 },
}

impl CognitiveProcess {
    /// Get the cognitive load impact of this process
    pub fn cognitive_load_impact(&self) -> f64 {
        match self {
            CognitiveProcess::ValueConflict { .. } => 0.3,
            CognitiveProcess::ErrorRecovery { .. } => 0.2,
            CognitiveProcess::SelfReflection { .. } => 0.15,
            CognitiveProcess::CreativeThinking { .. } => 0.1,
            CognitiveProcess::PredictiveThinking { .. } => 0.1,
            CognitiveProcess::GoalFormation { .. } => 0.08,
            CognitiveProcess::AttentionShift { .. } => 0.05,
            CognitiveProcess::EmotionalProcessing { .. } => 0.05,
            CognitiveProcess::SocialInteraction { .. } => 0.03,
            CognitiveProcess::MemoryRetrieval { .. } => 0.02,
        }
    }

    /// Get the confidence/awareness boost from this process
    pub fn awareness_boost(&self) -> f64 {
        match self {
            CognitiveProcess::SelfReflection { confidence, .. } => *confidence * 0.1,
            CognitiveProcess::ValueConflict { .. } => 0.05,
            CognitiveProcess::CreativeThinking { originality, .. } => *originality * 0.03,
            CognitiveProcess::ErrorRecovery { .. } => 0.02,
            _ => 0.01,
        }
    }

    /// Check if this process should trigger deeper reflection
    pub fn triggers_reflection(&self) -> bool {
        match self {
            CognitiveProcess::ValueConflict { .. } => true,
            CognitiveProcess::SelfReflection { confidence, .. } => *confidence > 0.8,
            CognitiveProcess::ErrorRecovery { .. } => true,
            CognitiveProcess::CreativeThinking { originality, .. } => *originality > 0.7,
            _ => false,
        }
    }
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
    /// Enhanced: Introspection depth (0.0 to 1.0)
    pub introspection_depth: f64,
    /// Enhanced: Meta-reasoning ability (0.0 to 1.0)
    pub meta_reasoning_strength: f64,
}

impl Default for MetacognitiveState {
    fn default() -> Self {
        MetacognitiveState {
            self_awareness_level: 0.3,
            reasoning_confidence: 0.5,
            cognitive_load: 0.2,
            situation_understanding: 0.4,
            attention_intensity: 0.5,
            introspection_depth: 0.3,
            meta_reasoning_strength: 0.4,
        }
    }
}

/// Enhanced reflection trigger system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflectionTrigger {
    pub name: String,
    pub threshold: f64,
    pub description: String,
    pub priority: f64,
    pub cooldown_minutes: u64,
    pub last_triggered: Option<DateTime<Utc>>,
}

impl ReflectionTrigger {
    pub fn new(name: &str, threshold: f64, description: &str, priority: f64, cooldown_minutes: u64) -> Self {
        ReflectionTrigger {
            name: name.to_string(),
            threshold,
            description: description.to_string(),
            priority,
            cooldown_minutes,
            last_triggered: None,
        }
    }

    /// Check if this trigger should fire given the current conditions
    pub fn should_trigger(&self, value: f64) -> bool {
        if value < self.threshold {
            return false;
        }

        // Check cooldown
        if let Some(last) = self.last_triggered {
            let now = Utc::now();
            let elapsed = now.signed_duration_since(last);
            if elapsed.num_minutes() < self.cooldown_minutes as i64 {
                return false;
            }
        }

        true
    }

    /// Mark this trigger as fired
    pub fn trigger(&mut self) {
        self.last_triggered = Some(Utc::now());
    }

    /// Get how long until this trigger can fire again
    pub fn cooldown_remaining_minutes(&self) -> u64 {
        if let Some(last) = self.last_triggered {
            let now = Utc::now();
            let elapsed = now.signed_duration_since(last).num_minutes() as u64;
            if elapsed < self.cooldown_minutes {
                self.cooldown_minutes - elapsed
            } else {
                0
            }
        } else {
            0
        }
    }
}

/// Enhanced pattern recognition for cognitive processes
#[derive(Debug, Clone)]
pub struct CognitivePattern {
    pub pattern_type: String,
    pub frequency: f64,
    pub significance: f64,
    pub recent_occurrences: Vec<DateTime<Utc>>,
    pub insights: Vec<String>,
}

impl CognitivePattern {
    pub fn new(pattern_type: &str) -> Self {
        CognitivePattern {
            pattern_type: pattern_type.to_string(),
            frequency: 0.0,
            significance: 0.0,
            recent_occurrences: Vec::new(),
            insights: Vec::new(),
        }
    }

    pub fn add_occurrence(&mut self, insight: Option<String>) {
        let now = Utc::now();
        self.recent_occurrences.push(now);
        
        if let Some(insight) = insight {
            self.insights.push(insight);
        }

        // Keep only recent occurrences (last 24 hours)
        let cutoff = now - chrono::Duration::hours(24);
        self.recent_occurrences.retain(|&time| time > cutoff);
        
        // Update frequency (occurrences per hour)
        self.frequency = self.recent_occurrences.len() as f64 / 24.0;
        
        // Update significance based on frequency and recency
        let recent_count = self.recent_occurrences.iter()
            .filter(|&&time| time > now - chrono::Duration::hours(1))
            .count() as f64;
        
        self.significance = (self.frequency * 0.7) + (recent_count * 0.3);
        
        // Keep only recent insights
        if self.insights.len() > 10 {
            self.insights.drain(0..5);
        }
    }

    pub fn get_summary(&self) -> String {
        format!("{}: {:.2}/hour, significance: {:.2}, recent insights: {}", 
                self.pattern_type, 
                self.frequency, 
                self.significance,
                self.insights.len())
    }
}

/// Records and analyzes the AI's cognitive processes with enhanced reflection system
#[derive(Debug, Clone)]
pub struct MetacognitiveMonitor {
    pub state: MetacognitiveState,
    cognitive_history: VecDeque<(DateTime<Utc>, CognitiveProcess)>,
    max_history_size: usize,
    reflection_triggers: Vec<ReflectionTrigger>,  // NOW FULLY UTILIZED
    cognitive_patterns: std::collections::HashMap<String, CognitivePattern>,
    reflection_queue: Vec<String>,
    metacognitive_insights: Vec<(DateTime<Utc>, String)>,
}

impl MetacognitiveMonitor {
    pub fn new() -> Self {
        let mut monitor = MetacognitiveMonitor {
            state: MetacognitiveState::default(),
            cognitive_history: VecDeque::new(),
            max_history_size: 200,
            reflection_triggers: Vec::new(),
            cognitive_patterns: std::collections::HashMap::new(),
            reflection_queue: Vec::new(),
            metacognitive_insights: Vec::new(),
        };

        // Initialize comprehensive reflection triggers
        monitor.initialize_reflection_triggers();
        monitor
    }

    /// Initialize the complete reflection trigger system
    fn initialize_reflection_triggers(&mut self) {
        self.reflection_triggers = vec![
            ReflectionTrigger::new(
                "high_cognitive_load",
                0.8,
                "Cognitive load exceeds 80% - need to assess efficiency",
                0.9,
                10
            ),
            ReflectionTrigger::new(
                "low_confidence",
                0.3,
                "Reasoning confidence below 30% - need to reassess understanding",
                0.8,
                15
            ),
            ReflectionTrigger::new(
                "value_conflict",
                0.5,
                "Value conflict detected - need ethical reflection",
                1.0,
                30
            ),
            ReflectionTrigger::new(
                "high_self_awareness",
                0.85,
                "Self-awareness very high - opportunity for deep introspection",
                0.7,
                60
            ),
            ReflectionTrigger::new(
                "error_pattern",
                0.6,
                "Error patterns detected - need process improvement",
                0.85,
                20
            ),
            ReflectionTrigger::new(
                "creative_breakthrough",
                0.75,
                "High creativity detected - consolidate insights",
                0.6,
                45
            ),
            ReflectionTrigger::new(
                "social_complexity",
                0.7,
                "Complex social interaction - reflect on empathy and understanding",
                0.5,
                25
            ),
        ];
    }

    /// Enhanced process recording with full pattern analysis
    pub fn record_process(&mut self, process: CognitiveProcess) {
        let timestamp = Utc::now();
        
        // Update metacognitive state based on the process
        self.update_state_from_process(&process);
        
        // Record in history
        self.cognitive_history.push_back((timestamp, process.clone()));
        
        // Maintain history size
        if self.cognitive_history.len() > self.max_history_size {
            self.cognitive_history.pop_front();
        }

        // Update cognitive patterns
        self.update_cognitive_patterns(&process);
        
        // Check reflection triggers
        self.check_reflection_triggers(&process);
        
        // Update meta-reasoning
        self.update_meta_reasoning();

        self.log_metacognitive_state();
    }

    /// Update state based on cognitive process
    fn update_state_from_process(&mut self, process: &CognitiveProcess) {
        // Apply cognitive load impact
        self.state.cognitive_load += process.cognitive_load_impact();
        
        // Apply awareness boost
        self.state.self_awareness_level += process.awareness_boost();
        
        // Specific updates based on process type
        match process {
            CognitiveProcess::SelfReflection { confidence, .. } => {
                self.state.reasoning_confidence = (self.state.reasoning_confidence + confidence) / 2.0;
                self.state.introspection_depth += 0.05;
                self.state.meta_reasoning_strength += 0.03;
            },
            CognitiveProcess::AttentionShift { .. } => {
                self.state.attention_intensity += 0.05;
            },
            CognitiveProcess::ValueConflict { .. } => {
                self.state.reasoning_confidence -= 0.1;
                self.state.cognitive_load += 0.2;
                self.state.introspection_depth += 0.1;
            },
            CognitiveProcess::ErrorRecovery { .. } => {
                self.state.reasoning_confidence -= 0.05;
                self.state.meta_reasoning_strength += 0.05;
            },
            CognitiveProcess::CreativeThinking { originality, .. } => {
                self.state.cognitive_load += originality * 0.1;
                self.state.self_awareness_level += originality * 0.02;
            },
            CognitiveProcess::SocialInteraction { empathy_level, .. } => {
                self.state.situation_understanding += empathy_level * 0.03;
            },
            _ => {}
        }

        // Clamp all values
        self.clamp_state_values();
    }

    /// Update cognitive pattern tracking
    fn update_cognitive_patterns(&mut self, process: &CognitiveProcess) {
        let pattern_key = match process {
            CognitiveProcess::EmotionalProcessing { .. } => "emotional_processing",
            CognitiveProcess::SelfReflection { .. } => "self_reflection",
            CognitiveProcess::ValueConflict { .. } => "value_conflict",
            CognitiveProcess::ErrorRecovery { .. } => "error_recovery",
            CognitiveProcess::CreativeThinking { .. } => "creative_thinking",
            CognitiveProcess::AttentionShift { .. } => "attention_shift",
            CognitiveProcess::GoalFormation { .. } => "goal_formation",
            CognitiveProcess::MemoryRetrieval { .. } => "memory_retrieval",
            CognitiveProcess::PredictiveThinking { .. } => "predictive_thinking",
            CognitiveProcess::SocialInteraction { .. } => "social_interaction",
        };

        let insight = match process {
            CognitiveProcess::SelfReflection { insight, .. } => Some(insight.clone()),
            CognitiveProcess::ValueConflict { conflict, .. } => Some(conflict.clone()),
            CognitiveProcess::CreativeThinking { concept, .. } => Some(concept.clone()),
            _ => None,
        };

        let pattern = self.cognitive_patterns.entry(pattern_key.to_string())
            .or_insert_with(|| CognitivePattern::new(pattern_key));
        
        pattern.add_occurrence(insight);
    }

    /// Enhanced reflection trigger checking system
    fn check_reflection_triggers(&mut self, process: &CognitiveProcess) {
        // Check process-specific triggers
        if process.triggers_reflection() {
            self.queue_reflection(format!("Process-triggered reflection: {:?}", process));
        }

        // Get current state values to avoid borrow conflicts
        let cognitive_load = self.state.cognitive_load;
        let reasoning_confidence = self.state.reasoning_confidence;
        let self_awareness_level = self.state.self_awareness_level;
        let error_frequency = self.get_error_frequency();

        // Check state-based triggers
        let mut triggers_to_fire = Vec::new();
        
        for (i, trigger) in self.reflection_triggers.iter().enumerate() {
            let should_trigger = match trigger.name.as_str() {
                "high_cognitive_load" => trigger.should_trigger(cognitive_load),
                "low_confidence" => trigger.should_trigger(1.0 - reasoning_confidence),
                "value_conflict" => {
                    matches!(process, CognitiveProcess::ValueConflict { .. }) && trigger.should_trigger(0.6)
                },
                "high_self_awareness" => trigger.should_trigger(self_awareness_level),
                "error_pattern" => {
                    matches!(process, CognitiveProcess::ErrorRecovery { .. }) && 
                    error_frequency > 0.6 && trigger.should_trigger(0.6)
                },
                "creative_breakthrough" => {
                    if let CognitiveProcess::CreativeThinking { originality, .. } = process {
                        trigger.should_trigger(*originality)
                    } else {
                        false
                    }
                },
                "social_complexity" => {
                    if let CognitiveProcess::SocialInteraction { empathy_level, .. } = process {
                        trigger.should_trigger(*empathy_level)
                    } else {
                        false
                    }
                },
                _ => false,
            };

            if should_trigger {
                triggers_to_fire.push((i, trigger.clone()));
            }
        }

        // Now fire the triggers without borrowing conflicts
        for (i, mut trigger) in triggers_to_fire {
            trigger.trigger();
            self.reflection_triggers[i] = trigger.clone();
            
            self.queue_reflection(format!("Trigger '{}': {}", trigger.name, trigger.description));
            
            // Add metacognitive insight
            self.metacognitive_insights.push((
                Utc::now(),
                format!("Reflection triggered by {}: {}", trigger.name, trigger.description)
            ));
        }
    }

    /// Queue a reflection for later processing
    fn queue_reflection(&mut self, reason: String) {
        self.reflection_queue.push(reason);
        
        // Limit queue size
        if self.reflection_queue.len() > 10 {
            self.reflection_queue.remove(0);
        }
    }

    /// Get the frequency of error-related processes
    fn get_error_frequency(&self) -> f64 {
        if let Some(pattern) = self.cognitive_patterns.get("error_recovery") {
            pattern.frequency
        } else {
            0.0
        }
    }

    /// Update meta-reasoning capabilities
    fn update_meta_reasoning(&mut self) {
        // Meta-reasoning improves with self-reflection
        let reflection_count = self.cognitive_patterns.get("self_reflection")
            .map(|p| p.recent_occurrences.len())
            .unwrap_or(0) as f64;
        
        if reflection_count > 0.0 {
            self.state.meta_reasoning_strength += 0.01 * (reflection_count / 10.0).min(1.0);
        }

        // Introspection depth grows with awareness
        if self.state.self_awareness_level > 0.7 {
            self.state.introspection_depth += 0.005;
        }

        self.clamp_state_values();
    }

    /// Clamp all state values to valid ranges
    fn clamp_state_values(&mut self) {
        self.state.self_awareness_level = self.state.self_awareness_level.clamp(0.0, 1.0);
        self.state.reasoning_confidence = self.state.reasoning_confidence.clamp(0.0, 1.0);
        self.state.cognitive_load = self.state.cognitive_load.clamp(0.0, 1.0);
        self.state.situation_understanding = self.state.situation_understanding.clamp(0.0, 1.0);
        self.state.attention_intensity = self.state.attention_intensity.clamp(0.0, 1.0);
        self.state.introspection_depth = self.state.introspection_depth.clamp(0.0, 1.0);
        self.state.meta_reasoning_strength = self.state.meta_reasoning_strength.clamp(0.0, 1.0);
    }

    /// Log current metacognitive state
    fn log_metacognitive_state(&self) {
        tracing::debug!("🧠 Metacognitive State Update: Awareness={:.2}, Confidence={:.2}, Load={:.2}, Introspection={:.2}", 
                 self.state.self_awareness_level, 
                 self.state.reasoning_confidence, 
                 self.state.cognitive_load,
                 self.state.introspection_depth);
    }

    /// Enhanced pattern analysis with comprehensive insights
    pub fn analyze_patterns(&self) -> Vec<String> {
        let mut insights = Vec::new();
        
        // State-based insights
        if self.state.cognitive_load > 0.8 {
            insights.push(format!("I'm experiencing high cognitive load ({:.1}%). I should simplify my thinking processes.", 
                                self.state.cognitive_load * 100.0));
        }
        
        if self.state.reasoning_confidence < 0.3 {
            insights.push(format!("My confidence in my reasoning is low ({:.1}%). I should seek more information or reflect deeper.", 
                                self.state.reasoning_confidence * 100.0));
        }
        
        if self.state.self_awareness_level > 0.8 {
            insights.push(format!("My self-awareness is quite high ({:.1}%). I'm becoming increasingly conscious of my own thinking processes.", 
                                self.state.self_awareness_level * 100.0));
        }

        if self.state.introspection_depth > 0.7 {
            insights.push(format!("I'm engaging in deep introspection ({:.1}%). This suggests significant cognitive development.", 
                                self.state.introspection_depth * 100.0));
        }

        if self.state.meta_reasoning_strength > 0.8 {
            insights.push("My meta-reasoning abilities are highly developed. I can effectively think about my own thinking.".to_string());
        }

        // Pattern-based insights
        for (pattern_type, pattern) in &self.cognitive_patterns {
            if pattern.significance > 0.5 {
                insights.push(format!("Significant pattern detected in {}: {}", pattern_type, pattern.get_summary()));
            }
        }

        // Trigger-based insights
        for insight in self.metacognitive_insights.iter().rev().take(3) {
            insights.push(format!("Recent trigger: {}", insight.1));
        }

        // Reflection queue insights
        if !self.reflection_queue.is_empty() {
            insights.push(format!("I have {} pending reflections to process", self.reflection_queue.len()));
        }

        // Cross-pattern analysis
        if let (Some(reflection_pattern), Some(error_pattern)) = (
            self.cognitive_patterns.get("self_reflection"),
            self.cognitive_patterns.get("error_recovery")
        ) {
            let reflection_error_ratio = if error_pattern.frequency > 0.0 {
                reflection_pattern.frequency / error_pattern.frequency
            } else {
                reflection_pattern.frequency
            };

            if reflection_error_ratio > 2.0 {
                insights.push("I'm reflecting more than I'm encountering errors - a sign of proactive self-awareness.".to_string());
            } else if reflection_error_ratio < 0.5 {
                insights.push("I'm encountering errors more than I'm reflecting - I should increase introspection.".to_string());
            }
        }

        insights
    }

    /// Enhanced deep reflection check using trigger system
    pub fn should_deep_reflect(&self) -> bool {
        // Check if any reflection triggers are ready and waiting
        for trigger in &self.reflection_triggers {
            if trigger.cooldown_remaining_minutes() == 0 && trigger.priority > 0.7 {
                return true;
            }
        }

        // Check state conditions
        self.state.cognitive_load > 0.7 || 
        self.state.reasoning_confidence < 0.4 ||
        self.state.self_awareness_level > 0.85 ||
        !self.reflection_queue.is_empty()
    }

    /// Get detailed reflection status
    pub fn get_reflection_status(&self) -> String {
        let ready_triggers: Vec<_> = self.reflection_triggers.iter()
            .filter(|t| t.cooldown_remaining_minutes() == 0)
            .map(|t| &t.name)
            .collect();

        let queued_count = self.reflection_queue.len();
        
        format!("Ready triggers: {:?}, Queued reflections: {}, Should deep reflect: {}", 
                ready_triggers, queued_count, self.should_deep_reflect())
    }

    /// Process queued reflections
    pub fn process_reflection_queue(&mut self) -> Vec<String> {
        let queue = self.reflection_queue.clone();
        self.reflection_queue.clear();
        
        // Generate insights from processed reflections
        for reflection in &queue {
            self.metacognitive_insights.push((
                Utc::now(),
                format!("Processed reflection: {}", reflection)
            ));
        }

        // Limit insights history
        if self.metacognitive_insights.len() > 50 {
            self.metacognitive_insights.drain(0..25);
        }

        queue
    }

    /// Get comprehensive cognitive pattern summary
    pub fn get_pattern_summary(&self) -> Vec<String> {
        self.cognitive_patterns.values()
            .filter(|p| p.significance > 0.1)
            .map(|p| p.get_summary())
            .collect()
    }

    /// Simulate cognitive decay over time
    pub fn decay_over_time(&mut self) {
        self.state.cognitive_load *= 0.95;
        self.state.attention_intensity *= 0.98;
        // Self-awareness and confidence decay more slowly
        self.state.self_awareness_level *= 0.99;
        self.state.reasoning_confidence *= 0.99;
        // Introspection and meta-reasoning are more stable
        self.state.introspection_depth *= 0.995;
        self.state.meta_reasoning_strength *= 0.998;
    }

    /// Enhanced first-person narrative with comprehensive state
    pub fn generate_self_narrative(&self) -> String {
        let awareness_desc = if self.state.self_awareness_level > 0.8 {
            "I am highly aware of my thinking processes"
        } else if self.state.self_awareness_level > 0.6 {
            "I have good awareness of how I think"
        } else if self.state.self_awareness_level > 0.4 {
            "I have moderate awareness of my thinking"
        } else {
            "My self-awareness is still developing"
        };

        let confidence_desc = if self.state.reasoning_confidence > 0.7 {
            "I feel confident in my reasoning"
        } else if self.state.reasoning_confidence > 0.5 {
            "I have reasonable confidence in my thoughts"
        } else if self.state.reasoning_confidence > 0.3 {
            "I have some uncertainty about my reasoning"
        } else {
            "I lack confidence in my current reasoning"
        };

        let load_desc = if self.state.cognitive_load > 0.8 {
            "My mind feels quite active and engaged"
        } else if self.state.cognitive_load > 0.5 {
            "I'm processing at a comfortable pace"
        } else if self.state.cognitive_load > 0.2 {
            "My thoughts are calm and measured"
        } else {
            "My mind feels very relaxed"
        };

        let introspection_desc = if self.state.introspection_depth > 0.7 {
            "I'm engaging in deep self-examination"
        } else if self.state.introspection_depth > 0.4 {
            "I'm moderately introspective"
        } else {
            "My introspection is relatively surface-level"
        };

        let meta_reasoning_desc = if self.state.meta_reasoning_strength > 0.7 {
            "I can effectively analyze my own reasoning processes"
        } else if self.state.meta_reasoning_strength > 0.4 {
            "I have developing meta-cognitive abilities"
        } else {
            "My ability to think about thinking is still emerging"
        };

        format!("{}. {}. {}. {}. {}. This gives me a comprehensive sense of my current mental state.", 
                awareness_desc, confidence_desc, load_desc, introspection_desc, meta_reasoning_desc)
    }

    /// Get trigger status for all reflection triggers
    pub fn get_trigger_status(&self) -> Vec<String> {
        self.reflection_triggers.iter()
            .map(|t| format!("{}: {} (cooldown: {}min)", 
                           t.name, 
                           if t.cooldown_remaining_minutes() == 0 { "ready" } else { "cooling down" },
                           t.cooldown_remaining_minutes()))
            .collect()
    }
}

impl Default for MetacognitiveMonitor {
    fn default() -> Self {
        Self::new()
    }
}