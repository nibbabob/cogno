// src/cogno_system.rs
//! COGNO System Integration
//!
//! The main orchestrator that combines cognitive appraisal, affective processing,
//! emotional expression, memory, personality, social context, and regulation
//! into a coherent and sophisticated emotion processing system.

use crate::cognitive_appraisal::{AppraisalInput, CognitiveAppraisal, OccEmotion};
use crate::core::AffectiveCore;
use crate::emotion_expression::EmotionExpression;
use crate::emotional_memory::{EmotionalMemory, EmotionalEpisode, EmotionalResolution};
use crate::emotional_regulation::{AdvancedEmotionRegulator, RegulationResult};
use crate::personality_traits::PersonalityProfile;
use crate::social_context::{SocialContextProcessor, SocialOutcome};
use std::time::Instant;

pub struct CognoSystem {
    cognitive_appraisal: CognitiveAppraisal,
    core: AffectiveCore,
    emotion_expression: EmotionExpression,
    emotional_memory: EmotionalMemory,
    personality: PersonalityProfile,
    social_context_processor: SocialContextProcessor,
    emotion_regulator: AdvancedEmotionRegulator,
}

impl CognoSystem {
    /// Create a new COGNO system with default personality
    pub fn new() -> Self {
        let personality = PersonalityProfile::new_default();
        Self::new_with_personality(personality)
    }
    
    /// Create a new COGNO system with custom personality
    pub fn new_with_personality(personality: PersonalityProfile) -> Self {
        let social_context_processor = SocialContextProcessor::new(personality.clone());
        let emotion_regulator = AdvancedEmotionRegulator::new(personality.clone());
        let mut core = AffectiveCore::new();
        core.baseline_state = personality.get_personality_baseline();

        CognoSystem {
            cognitive_appraisal: CognitiveAppraisal,
            core,
            emotion_expression: EmotionExpression,
            emotional_memory: EmotionalMemory::new(),
            personality: personality.clone(),
            social_context_processor,
            emotion_regulator,
        }
    }
    
    /// Process an emotional event through the complete pipeline
    pub fn process_emotional_event(&mut self, mut input: AppraisalInput) -> String {
        let start_time = Instant::now();
        let initial_state = self.core.current_state;

        // 1. Get contextual insights from emotional memory
        let insights = self.emotional_memory.get_contextual_insights(&input);

        // 2. Personality influences the initial appraisal
        self.personality.influence_appraisal(&mut input.context);

        // 3. Cognitive appraisal determines the initial emotion
        let mut emotion = self.cognitive_appraisal.appraise(&input);

        // 4. Social context enhances the appraisal and emotion
        let _social_influence = self.social_context_processor.enhance_social_appraisal(&mut input.context, &mut emotion);

        // 5. Personality influences the final emotion
        emotion = self.personality.influence_emotion(&mut emotion);

        // 6. Update the internal affective state based on the emotion
        self.core.process_emotion(&emotion);
        
        // 7. Apply natural emotional regulation (decay toward baseline)
        self.core.regulate_emotion();

        // 8. Apply advanced, strategic emotional regulation
        let regulation_result = self.emotion_regulator.regulate_emotion(
            &emotion, 
            &mut self.core.current_state, 
            insights.predicted_intensity
        );

        // 9. Update active regulation interventions
        self.emotion_regulator.update_interventions(&self.core.current_state);

        // 10. Generate the final emotional expression
        let mut expression = self.emotion_expression.express_emotion(&emotion, &self.core.current_state);

        // 11. Add regulation insights to expression if applicable
        if let RegulationResult::InterventionApplied { strategy, expected_effectiveness, .. } = &regulation_result {
            expression.push_str(&format!(" [Applying {:?} regulation strategy with {:.1}% expected effectiveness]", 
                strategy, expected_effectiveness * 100.0));
        }

        // 12. Record the episode in emotional memory
        let resolution = match regulation_result {
            RegulationResult::InterventionApplied { strategy, .. } => 
                EmotionalResolution::CognitiveReframing { 
                    new_appraisal: format!("Applied strategy: {:?}", strategy) 
                },
            RegulationResult::CapacityExhausted { .. } => 
                EmotionalResolution::Suppressed,
            _ => EmotionalResolution::NaturalDecay,
        };
        
        let episode = EmotionalEpisode {
            timestamp: start_time,
            trigger: input,
            emotion: emotion.clone(),
            initial_state,
            peak_state: self.core.current_state,
            duration: start_time.elapsed(),
            resolution,
            intensity_curve: vec![initial_state.valence, self.core.current_state.valence],
        };
        self.emotional_memory.record_episode(episode);

        // 13. Update social relationships if applicable
        if let Some(agent_name) = self.extract_agent_name_from_emotion(&emotion) {
            let outcome = self.infer_social_outcome(&emotion);
            self.social_context_processor.update_relationship(&agent_name, &emotion, outcome);
        }

        expression
    }

    /// Update social relationship explicitly
    pub fn update_social_relationship(&mut self, agent_name: &str, outcome: SocialOutcome) {
        // Create a dummy emotion for the update - in practice this would be the last emotion
        let dummy_emotion = OccEmotion::Joy { event: "interaction".to_string(), intensity: 0.5 };
        self.social_context_processor.update_relationship(agent_name, &dummy_emotion, outcome);
    }

    /// Display basic emotional state
    pub fn display_state(&self) {
        let state = &self.core.current_state;
        println!(
            "Valence: {:.2} ({})",
            state.valence,
            if state.valence > 0.0 { "positive" } else { "negative" }
        );
        println!(
            "Arousal: {:.2} ({})",
            state.arousal,
            if state.arousal > 0.5 { "energized" } else { "calm" }
        );
        println!(
            "Dominance: {:.2} ({})",
            state.dominance,
            if state.dominance > 0.0 { "in control" } else { "submissive" }
        );
        println!(
            "Recent emotions: {:?}",
            self.core.emotional_history
        );
    }

    /// Display personality profile
    pub fn display_personality(&self) {
        println!("Personality Profile:");
        println!("  Extraversion: {:.2} ({})", 
            self.personality.extraversion,
            if self.personality.extraversion > 0.6 { "outgoing" } 
            else if self.personality.extraversion > 0.4 { "balanced" } 
            else { "reserved" }
        );
        println!("  Agreeableness: {:.2} ({})", 
            self.personality.agreeableness,
            if self.personality.agreeableness > 0.6 { "cooperative" } 
            else if self.personality.agreeableness > 0.4 { "balanced" } 
            else { "competitive" }
        );
        println!("  Conscientiousness: {:.2} ({})", 
            self.personality.conscientiousness,
            if self.personality.conscientiousness > 0.6 { "organized" } 
            else if self.personality.conscientiousness > 0.4 { "balanced" } 
            else { "spontaneous" }
        );
        println!("  Neuroticism: {:.2} ({})", 
            self.personality.neuroticism,
            if self.personality.neuroticism > 0.6 { "sensitive" } 
            else if self.personality.neuroticism > 0.4 { "balanced" } 
            else { "stable" }
        );
        println!("  Openness: {:.2} ({})", 
            self.personality.openness,
            if self.personality.openness > 0.6 { "creative" } 
            else if self.personality.openness > 0.4 { "balanced" } 
            else { "traditional" }
        );
    }

    /// Display comprehensive system state
    pub fn display_comprehensive_state(&self) {
        println!("Current Affective State:");
        self.display_state();
        
        println!("\nEmotional Volatility: {:.2}", self.personality.get_emotional_volatility());
        println!("Recovery Rate: {:.2}%", self.personality.get_recovery_rate() * 100.0);
        
        let baseline = &self.core.baseline_state;
        println!("\nBaseline State (personality-influenced):");
        println!("  Valence: {:.2}, Arousal: {:.2}, Dominance: {:.2}", 
            baseline.valence, baseline.arousal, baseline.dominance);
    }

    /// Display emotional memory insights
    pub fn display_memory_insights(&self) {
        let stats = self.emotional_memory.get_emotional_statistics();
        println!("Total Episodes Recorded: {}", stats.total_episodes);
        println!("Average Intensity: {:.2}", stats.avg_intensity);
        
        if let Some(ref agent) = stats.most_common_agent {
            println!("Most Frequent Interaction: {}", agent);
        }
        
        println!("Emotional Patterns Learned: {}", stats.pattern_count);
        
        if !stats.dominant_patterns.is_empty() {
            println!("Dominant Patterns:");
            for (pattern, count) in stats.dominant_patterns.iter().take(3) {
                println!("  {} (triggered {} times)", pattern, count);
            }
        }
    }

    /// Display social insights
    pub fn display_social_insights(&self) {
        let key_people = ["Sarah", "John"]; // In practice, this would be dynamic
        
        for person in &key_people {
            if let Some(insights) = self.social_context_processor.get_social_insights(person) {
                println!("Relationship with {}:", person);
                println!("  Quality: {:.2}/1.0", insights.relationship_quality);
                println!("  Pattern: {}", insights.recent_pattern);
                println!("  Trajectory: {:.2}", insights.relationship_trajectory);
                println!("  Recommended Approach: {}", insights.recommended_approach);
                if insights.last_interaction_days < 1.0 {
                    println!("  Last Interaction: Today");
                } else {
                    println!("  Last Interaction: {:.1} days ago", insights.last_interaction_days);
                }
                println!();
            }
        }
    }

    /// Display regulation analytics
    pub fn display_regulation_analytics(&self) {
        let analytics = self.emotion_regulator.get_regulation_analytics();
        
        println!("Regulation Success Rate: {:.1}%", analytics.success_rate * 100.0);
        println!("Total Regulation Attempts: {}", analytics.total_attempts);
        println!("Current Regulation Capacity: {:.1}%", analytics.current_capacity * 100.0);
        println!("Active Interventions: {}", analytics.active_intervention_count);
        
        if analytics.regulation_fatigue > 0.3 {
            println!("⚠️  Regulation Fatigue: {:.1}% - Consider rest or external support", 
                analytics.regulation_fatigue * 100.0);
        }
        
        if let Some((strategy, effectiveness)) = analytics.most_effective_strategy {
            println!("Most Effective Strategy: {} ({:.1}% effective)", 
                strategy, effectiveness * 100.0);
        }

        let preferences = self.personality.get_regulation_preferences();
        println!("Preferred Regulation Strategies: {:?}", preferences);
    }

    /// Helper method to extract agent name from emotion for social updates
    fn extract_agent_name_from_emotion(&self, emotion: &OccEmotion) -> Option<String> {
        match emotion {
            OccEmotion::Gratitude { agent, .. } |
            OccEmotion::Anger { agent, .. } |
            OccEmotion::Admiration { agent, .. } |
            OccEmotion::Reproach { agent, .. } |
            OccEmotion::HappyFor { other: agent, .. } |
            OccEmotion::Pity { other: agent, .. } |
            OccEmotion::Gloating { other: agent, .. } |
            OccEmotion::Resentment { other: agent, .. } => Some(agent.clone()),
            _ => None,
        }
    }

    /// Infer social outcome from emotion type
    fn infer_social_outcome(&self, emotion: &OccEmotion) -> SocialOutcome {
        match emotion {
            OccEmotion::Gratitude { .. } |
            OccEmotion::Admiration { .. } |
            OccEmotion::HappyFor { .. } => SocialOutcome::PositiveInteraction,
            
            OccEmotion::Anger { .. } |
            OccEmotion::Reproach { .. } => SocialOutcome::Conflict,
            
            OccEmotion::Pity { .. } => SocialOutcome::NegativeInteraction,
            
            OccEmotion::Resentment { .. } |
            OccEmotion::Gloating { .. } => SocialOutcome::NegativeInteraction,
            
            _ => SocialOutcome::Neutral,
        }
    }

    /// Get current emotional state summary
    pub fn get_emotional_summary(&self) -> String {
        let state = &self.core.current_state;
        let intensity = (state.valence.abs() + state.arousal + state.dominance.abs()) / 3.0;
        
        let mood = if state.valence > 0.3 && state.arousal > 0.5 {
            "energetic and positive"
        } else if state.valence > 0.3 && state.arousal <= 0.5 {
            "calm and content"
        } else if state.valence < -0.3 && state.arousal > 0.5 {
            "agitated and distressed"
        } else if state.valence < -0.3 && state.arousal <= 0.5 {
            "low and withdrawn"
        } else if state.arousal > 0.7 {
            "highly activated"
        } else {
            "neutral and balanced"
        };

        format!("Currently feeling {} with {:.1}% intensity", mood, intensity * 100.0)
    }

    /// Reset system to baseline (useful for testing or fresh starts)
    pub fn reset_to_baseline(&mut self) {
        self.core.current_state = self.personality.get_personality_baseline();
        self.core.emotional_history.clear();
        // Note: We don't reset memory, personality, or relationships as these persist
    }
}