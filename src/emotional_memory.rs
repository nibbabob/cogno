//! Emotional Memory System
//! 
//! Tracks emotional patterns, learns from outcomes, and provides contextual
//! emotional history for more nuanced future appraisals.

use crate::cognitive_appraisal::{OccEmotion, AppraisalInput};
use crate::core::AffectiveState;
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

/// Represents a complete emotional episode with context and outcome
#[derive(Debug, Clone)]
pub struct EmotionalEpisode {
    pub timestamp: Instant,
    pub trigger: AppraisalInput,
    pub emotion: OccEmotion,
    pub initial_state: AffectiveState,
    pub peak_state: AffectiveState,
    pub duration: Duration,
    pub resolution: EmotionalResolution,
    pub intensity_curve: Vec<f64>, // How intensity changed over time
}

#[derive(Debug, Clone)]
pub enum EmotionalResolution {
    NaturalDecay,
    ExternalResolution { outcome: String, satisfaction: f64 },
    CognitiveReframing { new_appraisal: String },
    SocialSupport { agent: String },
    Suppressed,
}

/// Tracks emotional patterns and relationships between entities
#[derive(Debug)]
pub struct EmotionalMemory {
    // Episode storage
    recent_episodes: VecDeque<EmotionalEpisode>,
    significant_episodes: Vec<EmotionalEpisode>, // High-intensity or important episodes
    
    // Pattern recognition
    agent_emotional_history: HashMap<String, Vec<EmotionalEpisode>>,
    context_patterns: HashMap<String, EmotionalPattern>,
    
    // Learning parameters
    max_recent_episodes: usize,
    significance_threshold: f64,
    pattern_memory_decay: f64,
}

#[derive(Debug, Clone)]
pub struct EmotionalPattern {
    pub context_signature: String,
    pub typical_emotions: Vec<(OccEmotion, f64)>, // Emotion type and frequency
    pub average_intensity: f64,
    pub typical_duration: Duration,
    pub successful_regulations: Vec<String>, // What regulation strategies worked
    pub trigger_count: u32,
    pub last_occurrence: Instant,
}

impl EmotionalMemory {
    pub fn new() -> Self {
        EmotionalMemory {
            recent_episodes: VecDeque::new(),
            significant_episodes: Vec::new(),
            agent_emotional_history: HashMap::new(),
            context_patterns: HashMap::new(),
            max_recent_episodes: 50,
            significance_threshold: 0.7,
            pattern_memory_decay: 0.95, // How much patterns decay over time
        }
    }

    /// Record a new emotional episode
    pub fn record_episode(&mut self, episode: EmotionalEpisode) {
        let intensity = self.calculate_episode_intensity(&episode);
        
        // Add to recent episodes
        self.recent_episodes.push_back(episode.clone());
        if self.recent_episodes.len() > self.max_recent_episodes {
            self.recent_episodes.pop_front();
        }
        
        // Check if this episode is significant enough to store long-term
        if intensity > self.significance_threshold {
            self.significant_episodes.push(episode.clone());
        }
        
        // Update agent-specific emotional history
        if let Some(agent_name) = self.extract_agent_name(&episode.trigger) {
            self.agent_emotional_history
                .entry(agent_name)
                .or_insert_with(Vec::new)
                .push(episode.clone());
        }
        
        // Update pattern recognition
        self.update_patterns(&episode);
    }

    /// Analyze emotional patterns and provide insights for future appraisals
    pub fn get_contextual_insights(&self, input: &AppraisalInput) -> ContextualInsights {
        let context_key = self.generate_context_key(input);
        
        let similar_episodes = self.find_similar_episodes(input);
        let agent_history = self.get_agent_emotional_history(input);
        let predicted_intensity = self.predict_emotional_intensity(input);
        let regulation_suggestions = self.suggest_regulation_strategies(input);
        
        ContextualInsights {
            similar_episodes,
            agent_history,
            predicted_intensity,
            regulation_suggestions,
            pattern_confidence: self.calculate_pattern_confidence(&context_key),
        }
    }

    /// Find episodes similar to current input
    fn find_similar_episodes(&self, input: &AppraisalInput) -> Vec<EmotionalEpisode> {
        let mut similar_with_scores = Vec::new();
        
        for episode in &self.recent_episodes {
            let similarity = self.calculate_similarity(input, &episode.trigger);
            if similarity > 0.6 {
                similar_with_scores.push((episode.clone(), similarity));
            }
        }
        
        // Sort by similarity (most similar first)
        similar_with_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        similar_with_scores.into_iter().take(5).map(|(episode, _)| episode).collect()
    }

    /// Calculate similarity between two appraisal inputs
    fn calculate_similarity(&self, input1: &AppraisalInput, input2: &AppraisalInput) -> f64 {
        let ctx1 = &input1.context;
        let ctx2 = &input2.context;
        
        let mut similarity = 0.0;
        let mut factors = 0.0;
        
        // Event type similarity
        if ctx1.event_type == ctx2.event_type {
            similarity += 0.3;
        }
        factors += 0.3;
        
        // Agent similarity
        if ctx1.agent == ctx2.agent {
            similarity += 0.25;
        }
        factors += 0.25;
        
        // Goal relevance similarity
        if ctx1.is_goal_relevant == ctx2.is_goal_relevant {
            similarity += 0.2;
        }
        factors += 0.2;
        
        // Intensity similarity (continuous)
        let intensity_diff = (ctx1.intensity - ctx2.intensity).abs();
        similarity += (1.0 - intensity_diff) * 0.15;
        factors += 0.15;
        
        // Probability similarity
        let prob_diff = (ctx1.probability - ctx2.probability).abs();
        similarity += (1.0 - prob_diff) * 0.1;
        factors += 0.1;
        
        similarity / factors
    }

    /// Predict emotional intensity based on historical patterns
    fn predict_emotional_intensity(&self, input: &AppraisalInput) -> f64 {
        let context_key = self.generate_context_key(input);
        
        if let Some(pattern) = self.context_patterns.get(&context_key) {
            // Adjust based on pattern frequency and recency
            let recency_factor = self.calculate_recency_factor(pattern.last_occurrence);
            pattern.average_intensity * recency_factor
        } else {
            // No historical pattern, use input intensity
            input.context.intensity
        }
    }

    /// Suggest emotion regulation strategies based on past success
    fn suggest_regulation_strategies(&self, input: &AppraisalInput) -> Vec<String> {
        let context_key = self.generate_context_key(input);
        
        if let Some(pattern) = self.context_patterns.get(&context_key) {
            pattern.successful_regulations.clone()
        } else {
            // Default strategies based on emotion type prediction
            self.get_default_regulation_strategies(input)
        }
    }

    fn get_default_regulation_strategies(&self, input: &AppraisalInput) -> Vec<String> {
        match input.context.is_goal_relevant {
            crate::cognitive_appraisal::GoalRelevance::Harmful => vec![
                "Cognitive reappraisal: Focus on learning opportunities".to_string(),
                "Problem-solving: Identify actionable steps".to_string(),
                "Acceptance: Acknowledge limitations of control".to_string(),
            ],
            crate::cognitive_appraisal::GoalRelevance::Beneficial => vec![
                "Savoring: Consciously appreciate positive aspects".to_string(),
                "Gratitude practice: Acknowledge contributing factors".to_string(),
                "Sharing: Express positive experience with others".to_string(),
            ],
            _ => vec![
                "Mindful observation: Notice emotions without judgment".to_string(),
                "Perspective-taking: Consider multiple viewpoints".to_string(),
            ],
        }
    }

    fn generate_context_key(&self, input: &AppraisalInput) -> String {
        format!("{:?}-{:?}-{:?}", 
            input.context.event_type,
            input.context.agent,
            input.context.is_goal_relevant
        )
    }

    fn calculate_episode_intensity(&self, episode: &EmotionalEpisode) -> f64 {
        // Combine initial intensity with peak emotional state
        let peak_intensity = (episode.peak_state.valence.abs() + 
                             episode.peak_state.arousal + 
                             episode.peak_state.dominance.abs()) / 3.0;
        
        (episode.trigger.context.intensity + peak_intensity) / 2.0
    }

    fn extract_agent_name(&self, input: &AppraisalInput) -> Option<String> {
        match &input.context.agent {
            crate::cognitive_appraisal::Agent::Other(name) => Some(name.clone()),
            crate::cognitive_appraisal::Agent::Self_ => Some("Self".to_string()),
            crate::cognitive_appraisal::Agent::Unknown => None,
        }
    }

    fn get_agent_emotional_history(&self, input: &AppraisalInput) -> Vec<EmotionalEpisode> {
        if let Some(agent_name) = self.extract_agent_name(input) {
            self.agent_emotional_history
                .get(&agent_name)
                .map(|episodes| episodes.iter().rev().take(10).cloned().collect())
                .unwrap_or_default()
        } else {
            Vec::new()
        }
    }

    fn update_patterns(&mut self, episode: &EmotionalEpisode) {
        let context_key = self.generate_context_key(&episode.trigger);
        let episode_intensity = self.calculate_episode_intensity(episode);

        let pattern = self
            .context_patterns
            .entry(context_key.clone())
            .or_insert_with(|| EmotionalPattern {
                context_signature: context_key,
                typical_emotions: Vec::new(),
                average_intensity: 0.0,
                typical_duration: Duration::from_secs(0),
                successful_regulations: Vec::new(),
                trigger_count: 0,
                last_occurrence: episode.timestamp,
            });

        // Update pattern statistics
        pattern.trigger_count += 1;
        pattern.last_occurrence = episode.timestamp;
        
        // Update average intensity (running average)
        pattern.average_intensity = 
            (pattern.average_intensity * (pattern.trigger_count - 1) as f64 + episode_intensity) 
            / pattern.trigger_count as f64;
        
        // Update typical duration (running average)
        let total_duration = pattern.typical_duration.as_secs_f64() * (pattern.trigger_count - 1) as f64
            + episode.duration.as_secs_f64();
        pattern.typical_duration = Duration::from_secs_f64(total_duration / pattern.trigger_count as f64);
        
        // Track successful regulation strategies
        if let EmotionalResolution::ExternalResolution { satisfaction, .. } = &episode.resolution {
            if *satisfaction > 0.7 {
                // This was a successful resolution - we should learn from it
                // In a real implementation, we'd track what specific strategy was used
            }
        }
    }

    fn calculate_pattern_confidence(&self, context_key: &str) -> f64 {
        if let Some(pattern) = self.context_patterns.get(context_key) {
            // Confidence based on frequency and recency
            let frequency_factor = (pattern.trigger_count as f64 / 10.0).min(1.0);
            let recency_factor = self.calculate_recency_factor(pattern.last_occurrence);
            (frequency_factor + recency_factor) / 2.0
        } else {
            0.0
        }
    }

    fn calculate_recency_factor(&self, last_occurrence: Instant) -> f64 {
        let elapsed = last_occurrence.elapsed().as_secs_f64();
        let days_elapsed = elapsed / (24.0 * 60.0 * 60.0);
        
        // Exponential decay - recent patterns are more relevant
        (-(days_elapsed / 30.0)).exp() // 30-day half-life
    }

    /// Get statistics about emotional patterns
    pub fn get_emotional_statistics(&self) -> EmotionalStatistics {
        let total_episodes = self.recent_episodes.len();
        let avg_intensity = self.recent_episodes.iter()
            .map(|e| self.calculate_episode_intensity(e))
            .sum::<f64>() / total_episodes.max(1) as f64;
        
        let most_common_agent = self.agent_emotional_history.iter()
            .max_by_key(|(_, episodes)| episodes.len())
            .map(|(name, _)| name.clone());
        
        let dominant_patterns = self.context_patterns.iter()
            .filter(|(_, pattern)| pattern.trigger_count > 2)
            .map(|(key, pattern)| (key.clone(), pattern.trigger_count))
            .collect();

        EmotionalStatistics {
            total_episodes,
            avg_intensity,
            most_common_agent,
            dominant_patterns,
            pattern_count: self.context_patterns.len(),
        }
    }
}

#[derive(Debug)]
pub struct ContextualInsights {
    pub similar_episodes: Vec<EmotionalEpisode>,
    pub agent_history: Vec<EmotionalEpisode>,
    pub predicted_intensity: f64,
    pub regulation_suggestions: Vec<String>,
    pub pattern_confidence: f64,
}

#[derive(Debug)]
pub struct EmotionalStatistics {
    pub total_episodes: usize,
    pub avg_intensity: f64,
    pub most_common_agent: Option<String>,
    pub dominant_patterns: Vec<(String, u32)>,
    pub pattern_count: usize,
}