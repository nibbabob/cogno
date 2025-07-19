//! Social Context Processor
//! 
//! Handles complex social emotions, relationship dynamics, and group contexts
//! that influence emotional processing beyond simple agent-based appraisals.

use crate::cognitive_appraisal::{AppraisalContext, OccEmotion, Agent};
use crate::personality_traits::PersonalityProfile;
use std::collections::HashMap;
use std::time::Instant;

/// Represents a relationship with another agent
#[derive(Debug, Clone)]
pub struct SocialRelationship {
    pub agent_name: String,
    pub relationship_type: RelationshipType,
    pub closeness: f64, // 0.0 to 1.0
    pub trust_level: f64, // 0.0 to 1.0
    pub power_dynamic: PowerDynamic,
    pub emotional_history: Vec<SocialEmotionalEpisode>,
    pub interaction_frequency: f64, // interactions per week
    pub last_interaction: Option<Instant>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RelationshipType {
    Family,
    Friend,
    Colleague,
    Supervisor,
    Subordinate,
    Romantic,
    Acquaintance,
    Stranger,
    Adversary,
}

#[derive(Debug, Clone)]
pub enum PowerDynamic {
    Equal,
    Higher, // We have more power/status
    Lower,  // They have more power/status
    Contextual(String), // Power varies by context
}

#[derive(Debug, Clone)]
pub struct SocialEmotionalEpisode {
    pub timestamp: Instant,
    pub emotion: OccEmotion,
    pub context: String,
    pub outcome: SocialOutcome,
    pub relationship_impact: f64, // -1.0 to 1.0
}

#[derive(Debug, Clone)]
pub enum SocialOutcome {
    PositiveInteraction,
    NegativeInteraction,
    Conflict,
    Reconciliation,
    Neutral,
    Avoided,
}

/// Group dynamics that affect emotional processing
#[derive(Debug, Clone)]
pub struct GroupContext {
    pub group_name: String,
    pub group_size: usize,
    pub our_role: GroupRole,
    pub group_cohesion: f64, // 0.0 to 1.0
    pub social_norms: Vec<SocialNorm>,
    pub current_tension: f64, // 0.0 to 1.0
}

#[derive(Debug, Clone)]
pub enum GroupRole {
    Leader,
    Member,
    Observer,
    Outsider,
}

#[derive(Debug, Clone)]
pub struct SocialNorm {
    pub description: String,
    pub enforcement_level: f64, // How strictly enforced
    pub personal_alignment: f64, // How much we agree with it
}

pub struct SocialContextProcessor {
    relationships: HashMap<String, SocialRelationship>,
    group_contexts: HashMap<String, GroupContext>,
    personality: PersonalityProfile,
    social_skills: SocialSkills,
    current_social_context: Option<String>, // Current group/setting
}

#[derive(Debug, Clone)]
pub struct SocialSkills {
    pub empathy: f64,
    pub social_perception: f64,
    pub communication: f64,
    pub conflict_resolution: f64,
    pub influence: f64,
}

impl SocialContextProcessor {
    pub fn new(personality: PersonalityProfile) -> Self {
        let social_skills = SocialSkills::from_personality(&personality);
        
        SocialContextProcessor {
            relationships: HashMap::new(),
            group_contexts: HashMap::new(),
            personality,
            social_skills,
            current_social_context: None,
        }
    }

    /// Process social aspects of an emotional appraisal
    pub fn enhance_social_appraisal(&mut self, context: &mut AppraisalContext, emotion: &mut OccEmotion) -> SocialInfluence {
        let mut influences = Vec::new();
        
        // Analyze relationship context
        if let Agent::Other(agent_name) = &context.agent {
            if let Some(relationship) = self.relationships.get(agent_name) {
                let relationship_influence = self.apply_relationship_influence(context, emotion, relationship);
                influences.push(relationship_influence);
            } else {
                // Create new relationship entry for unknown agent
                let new_relationship = self.create_relationship_from_context(agent_name.clone(), context);
                self.relationships.insert(agent_name.clone(), new_relationship);
            }
        }

        // Apply group context if available
        if let Some(group_name) = &self.current_social_context {
            if let Some(group) = self.group_contexts.get(group_name) {
                let group_influence = self.apply_group_influence(context, emotion, group);
                influences.push(group_influence);
            }
        }

        // Apply general social anxiety/comfort
        let social_comfort_influence = self.apply_social_comfort_influence(context, emotion);
        influences.push(social_comfort_influence);

        SocialInfluence { influences }
    }

    /// Apply relationship-specific modifications to emotional processing
    fn apply_relationship_influence(
        &self, 
        context: &mut AppraisalContext, 
        emotion: &mut OccEmotion, 
        relationship: &SocialRelationship
    ) -> InfluenceType {
        let relationship_type = relationship.relationship_type.clone();
        let closeness = relationship.closeness;
        let trust_level = relationship.trust_level;
        let social_anxiety = self.personality.social_anxiety;
        
        match &relationship_type {
            RelationshipType::Family => {
                // Family relationships amplify emotional intensity
                context.intensity *= 1.0 + (closeness * 0.3);
                
                // Convert some negative emotions to more complex family emotions
                if let OccEmotion::Anger { agent, harmful_action: _ } = emotion {
                    if closeness > 0.7 {
                        // Close family anger might become disappointment
                        *emotion = OccEmotion::Disappointment { 
                            failed_hope: format!("Expected better from family member {}", agent)
                        };
                    }
                }
            },
            
            RelationshipType::Romantic => {
                // Romantic relationships have very high emotional stakes
                context.intensity *= 1.0 + (closeness * 0.5);
                
                // Romantic contexts amplify both positive and negative emotions
                match emotion {
                    OccEmotion::Joy { intensity, .. } => {
                        *intensity = (*intensity * (1.0 + closeness * 0.3)).clamp(0.0, 1.0);
                    },
                    OccEmotion::Distress { intensity, .. } => {
                        *intensity = (*intensity * (1.0 + closeness * 0.4)).clamp(0.0, 1.0);
                    },
                    _ => {}
                }
            },
            
            RelationshipType::Supervisor => {
                // Power dynamics affect emotional responses
                match &relationship.power_dynamic {
                    PowerDynamic::Lower => {
                        // They have power over us - increases anxiety and reduces expressed anger
                        if let OccEmotion::Anger { agent, harmful_action: _ } = emotion {
                            // Convert to anxiety or resentment instead
                            *emotion = OccEmotion::Fear { 
                                prospect: format!("Potential consequences from conflict with {}", agent),
                                likelihood: 0.6 
                            };
                        }
                        
                        // Increases social anxiety component
                        context.intensity *= 1.0 + (social_anxiety * 0.2);
                    },
                    _ => {}
                }
            },
            
            RelationshipType::Friend => {
                // Friends provide emotional buffer and support
                if trust_level > 0.7 {
                    // High trust friends reduce negative emotion intensity
                    match emotion {
                        OccEmotion::Distress { intensity, .. } |
                        OccEmotion::Fear { likelihood: intensity, .. } => {
                            *intensity *= 0.8; // Friends provide comfort
                        },
                        _ => {}
                    }
                }
            },
            
            RelationshipType::Stranger => {
                // Strangers trigger social anxiety and uncertainty
                context.intensity *= 1.0 + (social_anxiety * 0.3);
                context.certainty *= 0.8; // Less certain about stranger interactions
            },
            
            _ => {} // Other relationship types
        }

        InfluenceType::RelationshipDynamic {
            relationship_type,
            closeness,
            trust: trust_level,
        }
    }

    /// Apply group context modifications
    fn apply_group_influence(
        &self, 
        context: &mut AppraisalContext, 
        emotion: &mut OccEmotion, 
        group: &GroupContext
    ) -> InfluenceType {
        // Group size affects social anxiety
        let group_anxiety_factor = match group.group_size {
            1..=3 => 0.0,
            4..=8 => 0.1,
            9..=20 => 0.2,
            _ => 0.3,
        };
        
        context.intensity *= 1.0 + (self.personality.social_anxiety * group_anxiety_factor);

        // Role affects emotional responses
        match group.our_role {
            GroupRole::Leader => {
                // Leaders feel more responsibility and control
                context.intensity *= 1.1; // Higher stakes
                
                // Convert some distress to determination
                if let OccEmotion::Distress { event, .. } = emotion {
                    *emotion = OccEmotion::Hope { 
                        prospect: format!("Finding solution to: {}", event),
                        likelihood: 0.7 
                    };
                }
            },
            
            GroupRole::Outsider => {
                // Outsiders feel less control and more social anxiety
                context.intensity *= 1.0 + (self.personality.social_anxiety * 0.4);
                
                // Reduce positive emotions (impostor syndrome)
                if let OccEmotion::Pride { intensity, .. } = emotion {
                    *intensity *= 0.7;
                }
            },
            
            _ => {}
        }

        // Group cohesion affects emotional amplification
        if group.group_cohesion > 0.7 {
            // High cohesion groups amplify shared emotions
            match emotion {
                OccEmotion::Joy { intensity, .. } => {
                    *intensity = (*intensity * 1.2).clamp(0.0, 1.0);
                },
                OccEmotion::Distress { intensity, .. } => {
                    *intensity = (*intensity * 1.1).clamp(0.0, 1.0);
                },
                _ => {}
            }
        }

        InfluenceType::GroupDynamic {
            role: group.our_role.clone(),
            cohesion: group.group_cohesion,
            size: group.group_size,
        }
    }

    /// Apply social comfort/anxiety influences
    fn apply_social_comfort_influence(
        &self, 
        context: &mut AppraisalContext, 
        emotion: &mut OccEmotion
    ) -> InfluenceType {
        // Social anxiety affects all social interactions
        if let Agent::Other(_) = context.agent {
            let anxiety_amplification = self.personality.social_anxiety * 0.2;
            context.intensity *= 1.0 + anxiety_amplification;
            
            // High social anxiety can convert positive emotions to anxiety
            if self.personality.social_anxiety > 0.8 {
                match emotion {
                    OccEmotion::Admiration { agent, action: _ } => {
                        // Admiration might become anxiety about social comparison
                        *emotion = OccEmotion::Fear { 
                            prospect: format!("Being judged unfavorably compared to {}", agent),
                            likelihood: 0.5 
                        };
                    },
                    _ => {}
                }
            }
        }

        InfluenceType::SocialAnxiety {
            level: self.personality.social_anxiety,
            extroversion: self.personality.extraversion,
        }
    }

    /// Update relationship based on emotional episode
    pub fn update_relationship(&mut self, agent_name: &str, emotion: &OccEmotion, outcome: SocialOutcome) {
        let relationship_impact = self.calculate_relationship_impact(emotion, &outcome);
        if let Some(relationship) = self.relationships.get_mut(agent_name) {
            let episode = SocialEmotionalEpisode {
                timestamp: Instant::now(),
                emotion: emotion.clone(),
                context: "".to_string(), // Could be expanded
                outcome: outcome.clone(),
                relationship_impact,
            };

            // Update relationship parameters based on episode
            match outcome {
                SocialOutcome::PositiveInteraction => {
                    relationship.trust_level = (relationship.trust_level + 0.05).clamp(0.0, 1.0);
                    relationship.closeness = (relationship.closeness + 0.02).clamp(0.0, 1.0);
                },
                SocialOutcome::NegativeInteraction => {
                    relationship.trust_level = (relationship.trust_level - 0.1).clamp(0.0, 1.0);
                },
                SocialOutcome::Conflict => {
                    relationship.trust_level = (relationship.trust_level - 0.2).clamp(0.0, 1.0);
                    relationship.closeness = (relationship.closeness - 0.1).clamp(0.0, 1.0);
                },
                SocialOutcome::Reconciliation => {
                    relationship.trust_level = (relationship.trust_level + 0.15).clamp(0.0, 1.0);
                    relationship.closeness = (relationship.closeness + 0.1).clamp(0.0, 1.0);
                },
                _ => {}
            }

            relationship.emotional_history.push(episode);
            relationship.last_interaction = Some(Instant::now());
            
            // Limit history size
            if relationship.emotional_history.len() > 20 {
                relationship.emotional_history.remove(0);
            }
        }
    }

    fn calculate_relationship_impact(&self, emotion: &OccEmotion, outcome: &SocialOutcome) -> f64 {
        let base_impact: f64 = match emotion {
            OccEmotion::Gratitude { .. } => 0.8,
            OccEmotion::Admiration { .. } => 0.6,
            OccEmotion::Joy { .. } => 0.4,
            OccEmotion::Anger { .. } => -0.7,
            OccEmotion::Reproach { .. } => -0.5,
            OccEmotion::Disappointment { .. } => -0.3,
            _ => 0.0,
        };

        let outcome_modifier: f64 = match outcome {
            SocialOutcome::PositiveInteraction => 1.2,
            SocialOutcome::NegativeInteraction => 1.0,
            SocialOutcome::Conflict => 1.5,
            SocialOutcome::Reconciliation => 1.3,
            SocialOutcome::Avoided => 0.5,
            SocialOutcome::Neutral => 0.8,
        };

        (base_impact * outcome_modifier).clamp(-1.0, 1.0)
    }

    fn create_relationship_from_context(&self, agent_name: String, context: &AppraisalContext) -> SocialRelationship {
        // Infer relationship type from context clues
        let relationship_type = if agent_name.to_lowercase().contains("boss") || 
                                   agent_name.to_lowercase().contains("manager") {
            RelationshipType::Supervisor
        } else if context.is_praiseworthy && context.is_goal_relevant == crate::cognitive_appraisal::GoalRelevance::Beneficial {
            RelationshipType::Colleague
        } else {
            RelationshipType::Acquaintance
        };

        let power_dynamic = match relationship_type {
            RelationshipType::Supervisor => PowerDynamic::Lower,
            RelationshipType::Subordinate => PowerDynamic::Higher,
            _ => PowerDynamic::Equal,
        };

        SocialRelationship {
            agent_name,
            relationship_type,
            closeness: 0.3, // Default moderate closeness
            trust_level: 0.5, // Default neutral trust
            power_dynamic,
            emotional_history: Vec::new(),
            interaction_frequency: 1.0, // Default weekly interaction
            last_interaction: Some(Instant::now()),
        }
    }

    /// Get social insights for decision making
    pub fn get_social_insights(&self, agent_name: &str) -> Option<SocialInsights> {
        self.relationships.get(agent_name).map(|rel| {
            let recent_pattern = self.analyze_recent_emotional_pattern(rel);
            let relationship_trajectory = self.calculate_relationship_trajectory(rel);
            let recommended_approach = self.recommend_social_approach(rel);

            SocialInsights {
                relationship_quality: (rel.trust_level + rel.closeness) / 2.0,
                recent_pattern,
                relationship_trajectory,
                recommended_approach,
                interaction_frequency: rel.interaction_frequency,
                last_interaction_days: rel.last_interaction
                    .map(|t| t.elapsed().as_secs() as f64 / (24.0 * 60.0 * 60.0))
                    .unwrap_or(0.0),
            }
        })
    }

    fn analyze_recent_emotional_pattern(&self, relationship: &SocialRelationship) -> String {
        let recent_episodes: Vec<_> = relationship.emotional_history
            .iter()
            .rev()
            .take(5)
            .collect();

        if recent_episodes.is_empty() {
            return "No recent interactions".to_string();
        }

        let positive_count = recent_episodes.iter()
            .filter(|ep| matches!(ep.outcome, SocialOutcome::PositiveInteraction | SocialOutcome::Reconciliation))
            .count();

        let negative_count = recent_episodes.iter()
            .filter(|ep| matches!(ep.outcome, SocialOutcome::NegativeInteraction | SocialOutcome::Conflict))
            .count();

        match (positive_count, negative_count) {
            (p, n) if p > n * 2 => "Increasingly positive interactions".to_string(),
            (p, n) if n > p * 2 => "Pattern of tension or conflict".to_string(),
            _ => "Mixed interaction pattern".to_string(),
        }
    }

    fn calculate_relationship_trajectory(&self, relationship: &SocialRelationship) -> f64 {
        if relationship.emotional_history.len() < 3 {
            return 0.0; // Not enough data
        }

        let recent_impact: f64 = relationship.emotional_history
            .iter()
            .rev()
            .take(5)
            .map(|ep| ep.relationship_impact)
            .sum();

        recent_impact / 5.0 // Average impact of recent interactions
    }

    fn recommend_social_approach(&self, relationship: &SocialRelationship) -> String {
        match (relationship.trust_level, relationship.closeness) {
            (trust, close) if trust > 0.7 && close > 0.7 => 
                "Direct and open communication".to_string(),
            (trust, _) if trust < 0.3 => 
                "Careful, trust-building approach".to_string(),
            (_, close) if close < 0.3 => 
                "Professional and respectful distance".to_string(),
            _ => 
                "Balanced, moderately warm approach".to_string(),
        }
    }
}

impl SocialSkills {
    fn from_personality(personality: &PersonalityProfile) -> Self {
        SocialSkills {
            empathy: (personality.agreeableness + personality.emotional_intelligence) / 2.0,
            social_perception: (personality.emotional_intelligence + personality.openness) / 2.0,
            communication: (personality.extraversion + personality.emotional_intelligence) / 2.0,
            conflict_resolution: (personality.agreeableness + personality.emotional_intelligence + personality.conscientiousness) / 3.0,
            influence: (personality.extraversion + personality.conscientiousness) / 2.0,
        }
    }
}

#[derive(Debug)]
pub struct SocialInfluence {
    pub influences: Vec<InfluenceType>,
}

#[derive(Debug)]
pub enum InfluenceType {
    RelationshipDynamic { relationship_type: RelationshipType, closeness: f64, trust: f64 },
    GroupDynamic { role: GroupRole, cohesion: f64, size: usize },
    SocialAnxiety { level: f64, extroversion: f64 },
}

#[derive(Debug)]
pub struct SocialInsights {
    pub relationship_quality: f64,
    pub recent_pattern: String,
    pub relationship_trajectory: f64,
    pub recommended_approach: String,
    pub interaction_frequency: f64,
    pub last_interaction_days: f64,
}