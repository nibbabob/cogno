//! Advanced Emotion Regulation Module
//!
//! Implements sophisticated emotion regulation strategies that adapt based on
//! personality traits, emotional history, and context.

use crate::personality_traits::{PersonalityProfile, RegulationStrategy};
use crate::core::AffectiveState;
use crate::cognitive_appraisal::OccEmotion;
use std::collections::HashMap;
use std::time::Instant;

/// Represents an active regulation intervention
#[derive(Debug, Clone)]
pub struct RegulationIntervention {
    pub strategy: RegulationStrategy,
    pub start_time: Instant,
    pub target_emotion: String,
    pub initial_intensity: f64,
    pub expected_duration: std::time::Duration,
    pub effectiveness_so_far: f64,
}

/// Tracks the success and failure of regulation attempts
#[derive(Debug, Clone)]
pub struct RegulationOutcome {
    pub strategy_used: RegulationStrategy,
    pub emotion_type: String,
    pub initial_intensity: f64,
    pub final_intensity: f64,
    pub duration: std::time::Duration,
    pub success_rating: f64, // 0.0 to 1.0
    pub context_factors: Vec<String>,
}

pub struct AdvancedEmotionRegulator {
    /// Current active interventions
    active_interventions: Vec<RegulationIntervention>,
    
    /// History of regulation attempts for learning
    regulation_history: Vec<RegulationOutcome>,
    
    /// Strategy effectiveness learned over time
    strategy_effectiveness: HashMap<String, f64>, // strategy_emotion_key -> effectiveness
    
    /// Personality profile for strategy selection
    personality: PersonalityProfile,
    
    /// Current regulatory capacity (fatigue from overuse)
    regulatory_capacity: f64,
    
    /// Preferred regulation timing
    regulation_threshold: f64,
}

impl AdvancedEmotionRegulator {
    pub fn new(personality: PersonalityProfile) -> Self {
        AdvancedEmotionRegulator {
            active_interventions: Vec::new(),
            regulation_history: Vec::new(),
            strategy_effectiveness: HashMap::new(),
            personality,
            regulatory_capacity: 1.0,
            regulation_threshold: 0.6, // Regulate when intensity exceeds this
        }
    }

    /// Main regulation entry point - decides whether and how to regulate
    pub fn regulate_emotion(
        &mut self,
        emotion: &OccEmotion,
        current_state: &mut AffectiveState,
        context_stress: f64
    ) -> RegulationResult {
        // Check if regulation is needed
        if !self.should_regulate(current_state, context_stress) {
            return RegulationResult::NoActionNeeded;
        }

        // Check regulatory capacity (emotion regulation fatigue)
        if self.regulatory_capacity < 0.3 {
            return RegulationResult::CapacityExhausted {
                message: "Emotional regulation capacity is low. Consider rest or external support.".to_string()
            };
        }

        // Select the best regulation strategy
        let strategy = self.select_optimal_strategy(emotion, current_state, context_stress);
        
        // Apply the regulation strategy
        let intervention = self.apply_regulation_strategy(&strategy, emotion, current_state);
        
        // Record the intervention
        self.active_interventions.push(intervention.clone());
        
        // Reduce regulatory capacity
        self.regulatory_capacity *= 0.95;
        
        RegulationResult::InterventionApplied {
            strategy: strategy.clone(),
            intervention,
            expected_effectiveness: self.get_strategy_effectiveness(&strategy, emotion),
        }
    }

    /// Determine if emotional regulation is warranted
    fn should_regulate(&self, state: &AffectiveState, context_stress: f64) -> bool {
        // Calculate overall emotional intensity
        let intensity = (state.valence.abs() + state.arousal + state.dominance.abs()) / 3.0;
        
        // Regulate if:
        // 1. Intensity exceeds threshold
        // 2. High stress context requires active management
        // 3. Personality factors indicate need (high neuroticism)
        intensity > self.regulation_threshold ||
        context_stress > 0.7 ||
        (self.personality.neuroticism > 0.7 && intensity > 0.4)
    }

    /// Select the most appropriate regulation strategy
    fn select_optimal_strategy(
        &self,
        emotion: &OccEmotion,
        state: &AffectiveState,
        context_stress: f64
    ) -> RegulationStrategy {
        let personality_preferences = self.personality.get_regulation_preferences();
        let mut best_strategy = RegulationStrategy::Acceptance;
        let mut best_score = 0.0;

        for strategy in &personality_preferences {
            let score = self.calculate_strategy_score(strategy, emotion, state, context_stress);
            if score > best_score {
                best_score = score;
                best_strategy = strategy.clone();
            }
        }

        best_strategy
    }

    /// Calculate effectiveness score for a regulation strategy in current context
    fn calculate_strategy_score(
        &self,
        strategy: &RegulationStrategy,
        emotion: &OccEmotion,
        state: &AffectiveState,
        context_stress: f64
    ) -> f64 {
        let mut score = 0.0;

        // Base effectiveness for this emotion type
        score += strategy.effectiveness_for_emotion(emotion);

        // Historical effectiveness if we have data
        let strategy_key = self.get_strategy_emotion_key(strategy, emotion);
        if let Some(historical_effectiveness) = self.strategy_effectiveness.get(&strategy_key) {
            score += historical_effectiveness * 0.5; // Weight historical data
        }

        // Context-specific adjustments
        match strategy {
            RegulationStrategy::SocialSupport => {
                // Less effective if socially anxious or in high-stress contexts
                if self.personality.social_anxiety > 0.6 || context_stress > 0.8 {
                    score *= 0.7;
                }
            },
            
            RegulationStrategy::CognitiveReappraisal => {
                // More effective for people with high emotional intelligence
                if self.personality.emotional_intelligence > 0.7 {
                    score *= 1.3;
                }
                // Less effective when highly aroused
                if state.arousal > 0.8 {
                    score *= 0.6;
                }
            },
            
            RegulationStrategy::ProblemSolving => {
                // More effective for controllable situations
                if state.dominance > 0.0 {
                    score *= 1.2;
                } else {
                    score *= 0.5; // Can't solve what you can't control
                }
            },
            
            RegulationStrategy::Acceptance => {
                // More effective for uncontrollable situations
                if state.dominance < 0.0 {
                    score *= 1.4;
                }
            },
            
            RegulationStrategy::Distraction => {
                // Less effective for very intense emotions
                let intensity = (state.valence.abs() + state.arousal) / 2.0;
                if intensity > 0.8 {
                    score *= 0.4;
                }
            },
            
            _ => {} // No specific adjustments
        }

        // Regulatory capacity affects all strategies
        score *= self.regulatory_capacity;

        score
    }

    /// Apply the selected regulation strategy
    fn apply_regulation_strategy(
        &self,
        strategy: &RegulationStrategy,
        emotion: &OccEmotion,
        state: &mut AffectiveState
    ) -> RegulationIntervention {
        let initial_intensity = self.calculate_emotional_intensity(state);
        
        match strategy {
            RegulationStrategy::CognitiveReappraisal => {
                // Reduce negative valence by reframing
                if state.valence < 0.0 {
                    state.valence *= 0.7; // 30% reduction
                }
                state.arousal *= 0.8; // Calming effect
            },
            
            RegulationStrategy::ProblemSolving => {
                // Increase sense of control, reduce helplessness
                state.dominance += 0.3;
                state.dominance = state.dominance.clamp(-1.0, 1.0);
                state.arousal += 0.1; // Slight activation for action
            },
            
            RegulationStrategy::SocialSupport => {
                // Reduce negative valence, increase sense of connection
                if state.valence < 0.0 {
                    state.valence *= 0.6;
                }
                state.dominance += 0.2; // Support increases sense of agency
            },
            
            RegulationStrategy::Relaxation => {
                // Primary effect on arousal reduction
                state.arousal *= 0.5;
                if state.valence < 0.0 {
                    state.valence *= 0.9; // Mild positive effect
                }
            },
            
            RegulationStrategy::Distraction => {
                // Reduce overall intensity without changing valence direction
                state.valence *= 0.8;
                state.arousal *= 0.7;
            },
            
            RegulationStrategy::Acceptance => {
                // Reduces struggle, increases peace
                state.arousal *= 0.6;
                if state.dominance < -0.5 {
                    state.dominance += 0.3; // Acceptance increases sense of choice
                }
            },
            
            RegulationStrategy::Expressive => {
                // Temporary increase in intensity but faster resolution
                state.arousal += 0.2;
                state.arousal = state.arousal.clamp(0.0, 1.0);
                // (Actual benefit comes in faster decay over time)
            },
            
            RegulationStrategy::CreativeExpression => {
                // Transform negative emotions into creative energy
                if state.valence < 0.0 {
                    state.valence *= 0.8;
                    state.dominance += 0.2; // Creative agency
                }
            },
            
            RegulationStrategy::Mindfulness => {
                // Increases awareness and reduces reactivity
                state.arousal *= 0.7;
                state.novelty *= 0.8; // Reduces sense of overwhelming newness
            },
            
            RegulationStrategy::PerspectiveTaking => {
                // Similar to cognitive reappraisal but through empathy
                if state.valence < 0.0 {
                    state.valence *= 0.75;
                }
                state.novelty *= 0.9; // Other perspectives make things less novel
            },
            
            RegulationStrategy::Planning => {
                // Increases sense of control and preparedness
                state.dominance += 0.3;
                state.dominance = state.dominance.clamp(-1.0, 1.0);
                state.novelty *= 0.7; // Planning reduces uncertainty
            },
            
            RegulationStrategy::Suppression => {
                // Reduces expression but may maintain or increase internal intensity
                // This is generally less effective
                if state.arousal > 0.5 {
                    state.arousal -= 0.1; // Slight reduction in observable arousal
                }
                // Note: Suppression often has rebound effects (not modeled here)
            },
        }

        let expected_duration = self.estimate_strategy_duration(strategy, initial_intensity);
        
        RegulationIntervention {
            strategy: strategy.clone(),
            start_time: Instant::now(),
            target_emotion: format!("{:?}", emotion),
            initial_intensity,
            expected_duration,
            effectiveness_so_far: 0.0,
        }
    }

    /// Update active interventions and track outcomes
    pub fn update_interventions(&mut self, current_state: &AffectiveState) {
        let mut completed_indices = Vec::new();
        let current_intensity = self.calculate_emotional_intensity(current_state);

        self.active_interventions.iter_mut().enumerate().for_each(|(index, intervention)| {
            let elapsed = intervention.start_time.elapsed();
            let intensity_reduction = intervention.initial_intensity - current_intensity;
            intervention.effectiveness_so_far = (intensity_reduction / intervention.initial_intensity).clamp(0.0, 1.0);

            if elapsed >= intervention.expected_duration {
                completed_indices.push(index);
            }
        });

        for &index in completed_indices.iter().rev() {
            let intervention = self.active_interventions.remove(index);
            
            let outcome = RegulationOutcome {
                strategy_used: intervention.strategy.clone(),
                emotion_type: intervention.target_emotion.clone(),
                initial_intensity: intervention.initial_intensity,
                final_intensity: current_intensity,
                duration: intervention.start_time.elapsed(),
                success_rating: intervention.effectiveness_so_far,
                context_factors: Vec::new(),
            };
            
            self.regulation_history.push(outcome);
            self.update_strategy_effectiveness(
                &intervention.strategy, 
                &intervention.target_emotion, 
                intervention.effectiveness_so_far
            );
        }

        self.regulatory_capacity = (self.regulatory_capacity + 0.02).clamp(0.0, 1.0);
    }

    /// Learn from regulation outcomes to improve future strategy selection
    fn update_strategy_effectiveness(&mut self, strategy: &RegulationStrategy, emotion: &str, effectiveness: f64) {
        let key = format!("{:?}_{}", strategy, emotion);
        let current_effectiveness = self.strategy_effectiveness.get(&key).unwrap_or(&0.5);
        
        // Update with exponential moving average
        let alpha = 0.3; // Learning rate
        let new_effectiveness = current_effectiveness * (1.0 - alpha) + effectiveness * alpha;
        
        self.strategy_effectiveness.insert(key, new_effectiveness);
    }

    fn get_strategy_effectiveness(&self, strategy: &RegulationStrategy, emotion: &OccEmotion) -> f64 {
        let key = self.get_strategy_emotion_key(strategy, emotion);
        self.strategy_effectiveness.get(&key).cloned().unwrap_or(0.5)
    }

    fn get_strategy_emotion_key(&self, strategy: &RegulationStrategy, emotion: &OccEmotion) -> String {
        let emotion_type = format!("{:?}", emotion).split('{').next().unwrap_or("Unknown").to_string();
        format!("{:?}_{}", strategy, emotion_type)
    }

    fn calculate_emotional_intensity(&self, state: &AffectiveState) -> f64 {
        (state.valence.abs() + state.arousal + state.dominance.abs()) / 3.0
    }

    fn estimate_strategy_duration(&self, strategy: &RegulationStrategy, intensity: f64) -> std::time::Duration {
        let base_duration = match strategy {
            RegulationStrategy::Relaxation => 5, // minutes
            RegulationStrategy::CognitiveReappraisal => 15,
            RegulationStrategy::ProblemSolving => 30,
            RegulationStrategy::SocialSupport => 20,
            RegulationStrategy::Distraction => 10,
            RegulationStrategy::Acceptance => 25,
            RegulationStrategy::Expressive => 8,
            RegulationStrategy::CreativeExpression => 45,
            RegulationStrategy::Mindfulness => 20,
            RegulationStrategy::PerspectiveTaking => 15,
            RegulationStrategy::Planning => 35,
            RegulationStrategy::Suppression => 5,
        };
        
        // Adjust based on intensity - more intense emotions take longer to regulate
        let adjusted_duration = base_duration as f64 * (0.5 + intensity);
        std::time::Duration::from_secs((adjusted_duration * 60.0) as u64)
    }

    /// Get regulation analytics
    pub fn get_regulation_analytics(&self) -> RegulationAnalytics {
        let total_attempts = self.regulation_history.len();
        let successful_attempts = self.regulation_history.iter()
            .filter(|outcome| outcome.success_rating > 0.6)
            .count();
        
        let success_rate = if total_attempts > 0 {
            successful_attempts as f64 / total_attempts as f64
        } else {
            0.0
        };

        let most_effective_strategy = self.strategy_effectiveness.iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(key, effectiveness)| (key.clone(), *effectiveness));

        let current_capacity = self.regulatory_capacity;
        let active_intervention_count = self.active_interventions.len();

        RegulationAnalytics {
            success_rate,
            total_attempts,
            most_effective_strategy,
            current_capacity,
            active_intervention_count,
            regulation_fatigue: 1.0 - self.regulatory_capacity,
        }
    }
}

#[derive(Debug)]
pub enum RegulationResult {
    NoActionNeeded,
    CapacityExhausted { message: String },
    InterventionApplied {
        strategy: RegulationStrategy,
        intervention: RegulationIntervention,
        expected_effectiveness: f64,
    },
}

#[derive(Debug)]
pub struct RegulationAnalytics {
    pub success_rate: f64,
    pub total_attempts: usize,
    pub most_effective_strategy: Option<(String, f64)>,
    pub current_capacity: f64,
    pub active_intervention_count: usize,
    pub regulation_fatigue: f64,
}