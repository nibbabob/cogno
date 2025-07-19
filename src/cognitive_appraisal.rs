//! Cognitive Appraisal Module
//! 
//! Implements the OCC model's appraisal patterns to determine which emotions
//! should be triggered based on how events, actions, and objects are evaluated.

/// Represents all the information needed to appraise a situation
#[derive(Debug, Clone)]
pub struct AppraisalInput {
    pub description: String,
    pub context: AppraisalContext,
}

/// Detailed context for cognitive appraisal based on OCC theory
#[derive(Debug, Clone, Default)]
pub struct AppraisalContext {
    /// Whether this is about an event, action, or object
    pub event_type: EventType,
    /// Who is responsible (self, other, or unknown)
    pub agent: Agent,
    /// How this relates to personal goals
    pub is_goal_relevant: GoalRelevance,
    /// Whether an action is morally/socially praiseworthy
    pub is_praiseworthy: bool,
    /// Likelihood of the event (0.0 to 1.0)
    pub probability: f64,
    /// Confidence in the appraisal (0.0 to 1.0)
    pub certainty: f64,
    /// Whether this outcome was anticipated
    pub was_expected: bool,
    /// Intensity of goal relevance (0.0 to 1.0)
    pub intensity: f64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum EventType {
    #[default]
    Consequence,  // Results or outcomes
    Action,       // Deliberate behaviors
    Object,       // Things or concepts being evaluated
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Agent {
    #[default]
    Unknown,
    Self_,
    Other(String),
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum GoalRelevance {
    #[default]
    Neutral,
    Beneficial,   // Helps achieve goals
    Harmful,      // Hinders goals
}

/// The 22 emotion types from OCC theory, organized by their appraisal patterns
#[derive(Debug, Clone, PartialEq)]
pub enum OccEmotion {
    // === CONSEQUENCE-BASED EMOTIONS ===
    // Simple evaluation of events for self
    Joy { event: String, intensity: f64 },
    Distress { event: String, intensity: f64 },
    
    // Prospect-based (future-oriented)
    Hope { prospect: String, likelihood: f64 },
    Fear { prospect: String, likelihood: f64 },
    
    // Confirmation of prospects
    Satisfaction { confirmed_hope: String },
    Relief { averted_fear: String },
    Disappointment { failed_hope: String },
    FearConfirmed { realized_fear: String },
    
    // Consequences for others (empathy/schadenfreude)
    HappyFor { other: String, event: String },
    Pity { other: String, event: String },
    Gloating { other: String, event: String },
    Resentment { other: String, event: String },
    
    // === ACTION-BASED EMOTIONS ===
    // Self-agency emotions
    Pride { action: String, intensity: f64 },
    Shame { action: String, intensity: f64 },
    
    // Other-agency emotions  
    Admiration { agent: String, action: String },
    Reproach { agent: String, action: String },
    
    // === ATTRIBUTION EMOTIONS (Compound) ===
    // Joy/Distress + Agent Attribution
    Gratitude { agent: String, beneficial_action: String },
    Anger { agent: String, harmful_action: String },
    Gratification { own_beneficial_action: String },
    Remorse { own_harmful_action: String },
    
    // === OBJECT-BASED EMOTIONS ===
    Love { object: String },
    Hate { object: String },
}

pub struct CognitiveAppraisal;

impl CognitiveAppraisal {
    /// Core appraisal function that implements OCC decision tree
    pub fn appraise(&self, input: &AppraisalInput) -> OccEmotion {
        let ctx = &input.context;
        let description = &input.description;
        
        match ctx.event_type {
            EventType::Object => self.appraise_object(description, ctx),
            EventType::Action => self.appraise_action(description, ctx),
            EventType::Consequence => self.appraise_consequence(description, ctx),
        }
    }
    
    /// Appraise objects (things, concepts, people)
    fn appraise_object(&self, description: &str, ctx: &AppraisalContext) -> OccEmotion {
        match ctx.is_goal_relevant {
            GoalRelevance::Beneficial => {
                if ctx.intensity > 0.7 {
                    OccEmotion::Love { object: description.to_string() }
                } else {
                    OccEmotion::Joy { event: description.to_string(), intensity: ctx.intensity }
                }
            },
            GoalRelevance::Harmful => {
                if ctx.intensity > 0.7 {
                    OccEmotion::Hate { object: description.to_string() }
                } else {
                    OccEmotion::Distress { event: description.to_string(), intensity: ctx.intensity }
                }
            },
            GoalRelevance::Neutral => {
                // Neutral objects don't trigger strong emotions
                OccEmotion::Joy { event: "Neutral observation".to_string(), intensity: 0.1 }
            }
        }
    }
    
    /// Appraise actions (focusing on agency and praiseworthiness)
    fn appraise_action(&self, description: &str, ctx: &AppraisalContext) -> OccEmotion {
        match &ctx.agent {
            Agent::Self_ => {
                // Check if this action also had consequences for goals
                if ctx.is_goal_relevant != GoalRelevance::Neutral {
                    // Compound emotion: action evaluation + consequence
                    match (ctx.is_praiseworthy, ctx.is_goal_relevant) {
                        (true, GoalRelevance::Beneficial) => 
                            OccEmotion::Gratification { own_beneficial_action: description.to_string() },
                        (false, GoalRelevance::Harmful) => 
                            OccEmotion::Remorse { own_harmful_action: description.to_string() },
                        _ => {
                            // Simple action evaluation
                            if ctx.is_praiseworthy {
                                OccEmotion::Pride { action: description.to_string(), intensity: ctx.intensity }
                            } else {
                                OccEmotion::Shame { action: description.to_string(), intensity: ctx.intensity }
                            }
                        }
                    }
                } else {
                    // Pure action evaluation
                    if ctx.is_praiseworthy {
                        OccEmotion::Pride { action: description.to_string(), intensity: ctx.intensity }
                    } else {
                        OccEmotion::Shame { action: description.to_string(), intensity: ctx.intensity }
                    }
                }
            },
            Agent::Other(name) => {
                // Check if this action had consequences for our goals
                if ctx.is_goal_relevant != GoalRelevance::Neutral {
                    // Compound emotion: action evaluation + consequence attribution
                    match (ctx.is_praiseworthy, ctx.is_goal_relevant) {
                        (true, GoalRelevance::Beneficial) => 
                            OccEmotion::Gratitude { agent: name.clone(), beneficial_action: description.to_string() },
                        (false, GoalRelevance::Harmful) => 
                            OccEmotion::Anger { agent: name.clone(), harmful_action: description.to_string() },
                        _ => {
                            // Simple other-action evaluation
                            if ctx.is_praiseworthy {
                                OccEmotion::Admiration { agent: name.clone(), action: description.to_string() }
                            } else {
                                OccEmotion::Reproach { agent: name.clone(), action: description.to_string() }
                            }
                        }
                    }
                } else {
                    // Pure action evaluation
                    if ctx.is_praiseworthy {
                        OccEmotion::Admiration { agent: name.clone(), action: description.to_string() }
                    } else {
                        OccEmotion::Reproach { agent: name.clone(), action: description.to_string() }
                    }
                }
            },
            Agent::Unknown => {
                // Can't properly evaluate agency, default to consequence
                self.appraise_consequence(description, ctx)
            }
        }
    }
    
    /// Appraise consequences (events and their outcomes) - FIXED LOGIC
    fn appraise_consequence(&self, description: &str, ctx: &AppraisalContext) -> OccEmotion {
        if let Agent::Other(name) = &ctx.agent {
            return match (ctx.is_goal_relevant, ctx.is_praiseworthy) {
                (GoalRelevance::Beneficial, true) => OccEmotion::HappyFor { other: name.clone(), event: description.to_string() },
                (GoalRelevance::Beneficial, false) => OccEmotion::Resentment { other: name.clone(), event: description.to_string() },
                (GoalRelevance::Harmful, true) => OccEmotion::Gloating { other: name.clone(), event: description.to_string() },
                (GoalRelevance::Harmful, false) => OccEmotion::Pity { other: name.clone(), event: description.to_string() },
                _ => OccEmotion::Joy { event: description.to_string(), intensity: ctx.intensity },
            };
        }
        
        // Handle the special case of failed expectations first
        if ctx.probability == 0.0 && ctx.was_expected && ctx.is_goal_relevant == GoalRelevance::Beneficial {
            // This was something we hoped for but it didn't happen
            return OccEmotion::Disappointment { failed_hope: description.to_string() };
        }
        
        // Handle future/uncertain consequences (prospect-based emotions)
        if ctx.probability > 0.0 && ctx.probability < 1.0 {
            match ctx.is_goal_relevant {
                GoalRelevance::Beneficial => 
                    OccEmotion::Hope { prospect: description.to_string(), likelihood: ctx.probability },
                GoalRelevance::Harmful => 
                    OccEmotion::Fear { prospect: description.to_string(), likelihood: ctx.probability },
                GoalRelevance::Neutral => 
                    OccEmotion::Joy { event: "Neutral prospect".to_string(), intensity: 0.1 },
            }
        } else if ctx.probability == 1.0 {
            // Present/certain consequence - something that definitely happened
            if ctx.was_expected {
                // This was a confirmation of expectation
                match ctx.is_goal_relevant {
                    GoalRelevance::Beneficial => 
                        OccEmotion::Satisfaction { confirmed_hope: description.to_string() },
                    GoalRelevance::Harmful => 
                        OccEmotion::FearConfirmed { realized_fear: description.to_string() },
                    GoalRelevance::Neutral => 
                        OccEmotion::Joy { event: "Expected neutral outcome".to_string(), intensity: 0.2 },
                }
            } else {
                // Unexpected present consequence
                match ctx.is_goal_relevant {
                    GoalRelevance::Beneficial => 
                        OccEmotion::Joy { event: description.to_string(), intensity: ctx.intensity },
                    GoalRelevance::Harmful => 
                        OccEmotion::Distress { event: description.to_string(), intensity: ctx.intensity },
                    GoalRelevance::Neutral => 
                        OccEmotion::Joy { event: "Neutral event".to_string(), intensity: 0.1 },
                }
            }
        } else {
            // probability == 0.0 but not a failed expectation case
            // This means something definitely didn't happen
            match ctx.is_goal_relevant {
                GoalRelevance::Beneficial => 
                    // Missing out on something good
                    OccEmotion::Distress { event: format!("Missed opportunity: {}", description), intensity: ctx.intensity },
                GoalRelevance::Harmful => 
                    // Something bad was avoided - this is actually good!
                    OccEmotion::Relief { averted_fear: description.to_string() },
                GoalRelevance::Neutral => 
                    OccEmotion::Joy { event: "Neutral non-event".to_string(), intensity: 0.1 },
            }
        }
    }
}