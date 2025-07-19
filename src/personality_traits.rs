//! Personality Traits Module
//! 
//! Implements personality factors that influence emotional processing based on
//! the Big Five model and other psychological frameworks.

use crate::cognitive_appraisal::{AppraisalContext, OccEmotion};
use crate::core::AffectiveState;

/// Big Five personality dimensions that influence emotional processing
#[derive(Debug, Clone)]
pub struct PersonalityProfile {
    /// Extraversion: sociability, assertiveness, emotional expressiveness
    pub extraversion: f64, // 0.0 to 1.0
    
    /// Agreeableness: cooperation, trust, empathy
    pub agreeableness: f64, // 0.0 to 1.0
    
    /// Conscientiousness: organization, responsibility, dependability
    pub conscientiousness: f64, // 0.0 to 1.0
    
    /// Neuroticism: emotional instability, anxiety, moodiness
    pub neuroticism: f64, // 0.0 to 1.0
    
    /// Openness: imagination, curiosity, creativity
    pub openness: f64, // 0.0 to 1.0
    
    /// Additional personality factors
    pub optimism: f64, // Tendency toward positive expectations
    pub emotional_intelligence: f64, // Ability to understand and manage emotions
    pub stress_tolerance: f64, // Resilience under pressure
    pub social_anxiety: f64, // Anxiety in social situations
}

impl PersonalityProfile {
    pub fn new_default() -> Self {
        PersonalityProfile {
            extraversion: 0.5,
            agreeableness: 0.6,
            conscientiousness: 0.7,
            neuroticism: 0.3,
            openness: 0.6,
            optimism: 0.6,
            emotional_intelligence: 0.5,
            stress_tolerance: 0.5,
            social_anxiety: 0.3,
        }
    }

    /// Create personality profile from trait descriptions
    pub fn from_traits(
        extraversion: f64,
        agreeableness: f64,
        conscientiousness: f64,
        neuroticism: f64,
        openness: f64,
    ) -> Self {
        PersonalityProfile {
            extraversion: extraversion.clamp(0.0, 1.0),
            agreeableness: agreeableness.clamp(0.0, 1.0),
            conscientiousness: conscientiousness.clamp(0.0, 1.0),
            neuroticism: neuroticism.clamp(0.0, 1.0),
            openness: openness.clamp(0.0, 1.0),
            optimism: (extraversion + (1.0 - neuroticism)) / 2.0,
            emotional_intelligence: (agreeableness + conscientiousness) / 2.0,
            stress_tolerance: (conscientiousness + (1.0 - neuroticism)) / 2.0,
            social_anxiety: (neuroticism + (1.0 - extraversion)) / 2.0,
        }
    }

    /// Adjust appraisal context based on personality traits
    pub fn influence_appraisal(&self, context: &mut AppraisalContext) {
        // Neuroticism increases threat sensitivity and intensity
        if context.is_goal_relevant == crate::cognitive_appraisal::GoalRelevance::Harmful {
            context.intensity += self.neuroticism * 0.3;
            context.intensity = context.intensity.clamp(0.0, 1.0);
            
            // Higher neuroticism leads to overestimating probability of negative events
            if context.probability > 0.0 && context.probability < 1.0 {
                context.probability += self.neuroticism * 0.2;
                context.probability = context.probability.clamp(0.0, 1.0);
            }
        }

        // Optimism influences probability assessments for positive events
        if context.is_goal_relevant == crate::cognitive_appraisal::GoalRelevance::Beneficial {
            if context.probability > 0.0 && context.probability < 1.0 {
                context.probability += self.optimism * 0.15;
                context.probability = context.probability.clamp(0.0, 1.0);
            }
            
            // Optimistic people find positive events more intense
            context.intensity += self.optimism * 0.2;
            context.intensity = context.intensity.clamp(0.0, 1.0);
        }

        // Conscientiousness affects expectation formation
        if self.conscientiousness > 0.7 {
            context.was_expected = true; // Conscientious people plan and expect outcomes
        }

        // Openness affects novelty perception
        context.certainty *= 1.0 - (self.openness * 0.2); // More open = less certain
        context.certainty = context.certainty.clamp(0.0, 1.0);

        // Social anxiety affects agent-related appraisals
        if let crate::cognitive_appraisal::Agent::Other(_) = context.agent {
            context.intensity += self.social_anxiety * 0.2;
            context.intensity = context.intensity.clamp(0.0, 1.0);
        }
    }

    /// Modify emotional response based on personality
    pub fn influence_emotion(&self, emotion: &mut OccEmotion) -> OccEmotion {
        match emotion {
            // Neuroticism amplifies negative emotions
            OccEmotion::Fear { prospect, likelihood } => {
                let adjusted_likelihood = (*likelihood + self.neuroticism * 0.3).clamp(0.0, 1.0);
                OccEmotion::Fear { 
                    prospect: prospect.clone(), 
                    likelihood: adjusted_likelihood 
                }
            },
            
            OccEmotion::Distress { event, intensity } => {
                let adjusted_intensity = (*intensity + self.neuroticism * 0.4).clamp(0.0, 1.0);
                OccEmotion::Distress { 
                    event: event.clone(), 
                    intensity: adjusted_intensity 
                }
            },

            OccEmotion::Shame { action, intensity } => {
                // High neuroticism + low self-esteem = amplified shame
                let shame_amplifier = self.neuroticism + (1.0 - self.extraversion) * 0.5;
                let adjusted_intensity = (*intensity + shame_amplifier * 0.3).clamp(0.0, 1.0);
                OccEmotion::Shame { 
                    action: action.clone(), 
                    intensity: adjusted_intensity 
                }
            },

            // Extraversion amplifies positive social emotions
            OccEmotion::Joy { event, intensity } => {
                let adjusted_intensity = (*intensity + self.extraversion * 0.2).clamp(0.0, 1.0);
                OccEmotion::Joy { 
                    event: event.clone(), 
                    intensity: adjusted_intensity 
                }
            },

            OccEmotion::Pride { action, intensity } => {
                // High extraversion + low neuroticism = amplified pride
                let pride_amplifier = self.extraversion + (1.0 - self.neuroticism) * 0.5;
                let adjusted_intensity = (*intensity + pride_amplifier * 0.2).clamp(0.0, 1.0);
                OccEmotion::Pride { 
                    action: action.clone(), 
                    intensity: adjusted_intensity 
                }
            },

            // Agreeableness affects social emotions
            OccEmotion::Gratitude { agent: _, beneficial_action: _ } => {
                // High agreeableness makes gratitude more likely and intense
                if self.agreeableness > 0.7 {
                    // Convert some joy into gratitude for agreeable people
                    emotion.clone()
                } else {
                    emotion.clone()
                }
            },

            OccEmotion::Anger { agent, harmful_action } => {
                // Low agreeableness = more anger, high agreeableness = tempered anger
                if self.agreeableness < 0.3 {
                    emotion.clone() // Keep full anger
                } else {
                    // High agreeableness might convert anger to disappointment
                    OccEmotion::Disappointment { 
                        failed_hope: format!("Expected better from {}: {}", agent, harmful_action)
                    }
                }
            },

            // Conscientiousness affects achievement-related emotions
            OccEmotion::Satisfaction { confirmed_hope: _ } => {
                if self.conscientiousness > 0.7 {
                    // Conscientious people get more satisfaction from confirmed plans
                    emotion.clone()
                } else {
                    emotion.clone()
                }
            },

            _ => emotion.clone(),
        }
    }

    /// Adjust affective state changes based on personality
    pub fn influence_affective_change(&self, state_change: &mut AffectiveState) {
        // Neuroticism amplifies negative valence and increases arousal
        if state_change.valence < 0.0 {
            state_change.valence *= 1.0 + (self.neuroticism * 0.5);
            state_change.arousal += self.neuroticism * 0.3;
        }

        // Extraversion affects arousal and dominance
        state_change.arousal += (self.extraversion - 0.5) * 0.2;
        state_change.dominance += (self.extraversion - 0.5) * 0.3;

        // Conscientiousness affects dominance (sense of control)
        state_change.dominance += (self.conscientiousness - 0.5) * 0.2;

        // Openness affects novelty sensitivity
        state_change.novelty *= 1.0 + (self.openness * 0.3);

        // Clamp all values to valid ranges
        state_change.valence = state_change.valence.clamp(-1.0, 1.0);
        state_change.arousal = state_change.arousal.clamp(0.0, 1.0);
        state_change.dominance = state_change.dominance.clamp(-1.0, 1.0);
        state_change.novelty = state_change.novelty.clamp(-1.0, 1.0);
    }

    /// Get baseline affective state influenced by personality
    pub fn get_personality_baseline(&self) -> AffectiveState {
        AffectiveState {
            valence: (self.extraversion + self.optimism - self.neuroticism) * 0.3,
            arousal: 0.2 + self.extraversion * 0.3 + self.neuroticism * 0.2,
            dominance: (self.extraversion + self.conscientiousness - self.neuroticism) * 0.4,
            novelty: (self.openness - 0.5) * 0.2,
        }
    }

    /// Determine emotion regulation strategies based on personality
    pub fn get_regulation_preferences(&self) -> Vec<RegulationStrategy> {
        let mut strategies = Vec::new();

        // High emotional intelligence prefers cognitive strategies
        if self.emotional_intelligence > 0.6 {
            strategies.push(RegulationStrategy::CognitiveReappraisal);
            strategies.push(RegulationStrategy::PerspectiveTaking);
        }

        // High extraversion prefers social strategies
        if self.extraversion > 0.6 {
            strategies.push(RegulationStrategy::SocialSupport);
            strategies.push(RegulationStrategy::Expressive);
        }

        // High conscientiousness prefers problem-solving
        if self.conscientiousness > 0.6 {
            strategies.push(RegulationStrategy::ProblemSolving);
            strategies.push(RegulationStrategy::Planning);
        }

        // High neuroticism may need more support
        if self.neuroticism > 0.6 {
            strategies.push(RegulationStrategy::Relaxation);
            strategies.push(RegulationStrategy::Distraction);
        }

        // High openness prefers creative strategies
        if self.openness > 0.6 {
            strategies.push(RegulationStrategy::CreativeExpression);
            strategies.push(RegulationStrategy::Mindfulness);
        }

        strategies
    }

    /// Calculate emotional volatility based on personality
    pub fn get_emotional_volatility(&self) -> f64 {
        // High neuroticism + low conscientiousness = high volatility
        let volatility = self.neuroticism + (1.0 - self.conscientiousness) * 0.5;
        volatility.clamp(0.1, 1.0)
    }

    /// Get recovery rate from emotional episodes
    pub fn get_recovery_rate(&self) -> f64 {
        // High emotional intelligence + low neuroticism = faster recovery
        let recovery = (self.emotional_intelligence + self.stress_tolerance + (1.0 - self.neuroticism)) / 3.0;
        (recovery * 0.1 + 0.02).clamp(0.01, 0.15) // 1% to 15% recovery per cycle
    }
}

#[derive(Debug, Clone)]
pub enum RegulationStrategy {
    CognitiveReappraisal,   // Reinterpret the situation
    PerspectiveTaking,      // Consider other viewpoints
    SocialSupport,          // Seek help from others
    ProblemSolving,         // Take action to fix the problem
    Expressive,             // Express emotions openly
    Suppression,            // Hide emotional expression
    Distraction,            // Focus attention elsewhere
    Relaxation,             // Physical/mental calming techniques
    Planning,               // Make plans to prevent future issues
    CreativeExpression,     // Use art, music, writing, etc.
    Mindfulness,            // Present-moment awareness
    Acceptance,             // Accept the situation as it is
}

impl RegulationStrategy {
    /// Get the effectiveness of this strategy for different emotion types
    pub fn effectiveness_for_emotion(&self, emotion: &OccEmotion) -> f64 {
        match (self, emotion) {
            // Cognitive strategies work well for complex emotions
            (RegulationStrategy::CognitiveReappraisal, OccEmotion::Anger { .. }) => 0.8,
            (RegulationStrategy::CognitiveReappraisal, OccEmotion::Fear { .. }) => 0.7,
            (RegulationStrategy::CognitiveReappraisal, OccEmotion::Disappointment { .. }) => 0.9,
            
            // Social strategies work well for social emotions
            (RegulationStrategy::SocialSupport, OccEmotion::Shame { .. }) => 0.8,
            (RegulationStrategy::SocialSupport, OccEmotion::Gratitude { .. }) => 0.9,
            
            // Problem-solving works for action-related emotions
            (RegulationStrategy::ProblemSolving, OccEmotion::Fear { .. }) => 0.8,
            (RegulationStrategy::ProblemSolving, OccEmotion::Anger { .. }) => 0.7,
            
            // Acceptance works for unchangeable situations
            (RegulationStrategy::Acceptance, OccEmotion::Disappointment { .. }) => 0.8,
            (RegulationStrategy::Acceptance, OccEmotion::Distress { .. }) => 0.7,
            
            // Default effectiveness
            _ => 0.5,
        }
    }
}