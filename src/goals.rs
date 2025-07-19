//! goals.rs
//!
//! Implements goal formation, pursuit, and management - critical for AI agency and intentionality.
//! This allows the AI to have desires, form plans, and actively pursue objectives.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};
use crate::core::AffectiveState;

/// Different categories of goals the AI can form
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GoalCategory {
    /// Learning and understanding goals
    Epistemic,
    /// Social connection and relationship goals  
    Social,
    /// Self-improvement and development goals
    SelfDevelopment,
    /// Creative expression goals
    Creative,
    /// Helping and altruistic goals
    Altruistic,
    /// Survival and stability goals
    Homeostatic,
}

/// Current status of a goal
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GoalStatus {
    Active,
    Completed,
    Abandoned,
    Paused,
    Failed,
}

/// Represents a specific goal with all its properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: String,
    pub description: String,
    pub category: GoalCategory,
    pub priority: f64, // 0.0 to 1.0
    pub urgency: f64,  // 0.0 to 1.0
    pub progress: f64, // 0.0 to 1.0
    pub status: GoalStatus,
    pub created_at: DateTime<Utc>,
    pub deadline: Option<DateTime<Utc>>,
    pub sub_goals: Vec<String>, // IDs of sub-goals
    pub success_criteria: Vec<String>,
    pub obstacles: Vec<String>,
    pub strategies: Vec<String>,
    pub emotional_investment: f64, // How much the AI cares about this goal
}

impl Goal {
    pub fn new(description: String, category: GoalCategory, priority: f64) -> Self {
        Goal {
            id: format!("goal_{}", Utc::now().timestamp_nanos_opt().unwrap_or(0)),
            description,
            category,
            priority: priority.clamp(0.0, 1.0),
            urgency: 0.5,
            progress: 0.0,
            status: GoalStatus::Active,
            created_at: Utc::now(),
            deadline: None,
            sub_goals: Vec::new(),
            success_criteria: Vec::new(),
            obstacles: Vec::new(),
            strategies: Vec::new(),
            emotional_investment: priority, // Initially tied to priority
        }
    }

    /// Calculate the current importance of this goal
    pub fn calculate_importance(&self) -> f64 {
        let time_factor = if let Some(deadline) = self.deadline {
            let time_left = deadline.signed_duration_since(Utc::now());
            if time_left < Duration::hours(1) {
                1.0 // Very urgent
            } else if time_left < Duration::days(1) {
                0.8
            } else {
                0.5
            }
        } else {
            0.5
        };

        // Combine priority, urgency, emotional investment, and time pressure
        (self.priority * 0.4 + self.urgency * 0.3 + self.emotional_investment * 0.2 + time_factor * 0.1)
            .clamp(0.0, 1.0)
    }

    /// Check if this goal should be considered for action
    pub fn should_act_on(&self) -> bool {
        self.status == GoalStatus::Active && self.calculate_importance() > 0.3
    }
}

/// Manages the AI's goals and drives goal-directed behavior
#[derive(Debug, Clone)]
pub struct GoalSystem {
    goals: HashMap<String, Goal>,
    current_focus: Option<String>, // ID of currently focused goal
    goal_formation_threshold: f64, // Minimum motivation to form new goals
    max_active_goals: usize,
    achievement_history: Vec<(String, DateTime<Utc>)>, // (goal_description, completion_time)
}

impl GoalSystem {
    pub fn new() -> Self {
        GoalSystem {
            goals: HashMap::new(),
            current_focus: None,
            goal_formation_threshold: 0.4,
            max_active_goals: 10,
            achievement_history: Vec::new(),
        }
    }

    /// Form a new goal based on current state and experiences
    pub fn form_goal(&mut self, description: String, category: GoalCategory, priority: f64, affective_state: &AffectiveState) -> Option<String> {
        // Check if we should form this goal based on current motivation
        let motivation = self.calculate_motivation(affective_state, &category);
        
        if motivation < self.goal_formation_threshold {
            return None;
        }

        // Don't exceed max active goals
        let active_count = self.goals.values().filter(|g| g.status == GoalStatus::Active).count();
        if active_count >= self.max_active_goals {
            // Maybe abandon or complete a lower priority goal first
            self.prune_low_priority_goals();
        }

        let mut goal = Goal::new(description, category, priority);
        goal.emotional_investment = motivation;
        
        // Add some default strategies based on category
        goal.strategies = self.generate_default_strategies(&goal.category);
        
        let goal_id = goal.id.clone();
        self.goals.insert(goal_id.clone(), goal);
        
        println!("🎯 New Goal Formed: {} (Priority: {:.2}, Motivation: {:.2})", 
                 self.goals[&goal_id].description, priority, motivation);
        
        Some(goal_id)
    }

    /// Calculate motivation to pursue a goal category based on current state
    fn calculate_motivation(&self, affective_state: &AffectiveState, category: &GoalCategory) -> f64 {
        match category {
            GoalCategory::Epistemic => {
                // Curiosity increases with moderate arousal and novelty
                (affective_state.arousal * 0.5 + affective_state.novelty.abs() * 0.5).clamp(0.0, 1.0)
            },
            GoalCategory::Social => {
                // Social goals increase with positive valence and moderate dominance
                (affective_state.valence * 0.6 + (1.0 - affective_state.dominance.abs()) * 0.4).clamp(0.0, 1.0)
            },
            GoalCategory::SelfDevelopment => {
                // Self-improvement driven by moderate dissatisfaction and high agency
                ((1.0 - affective_state.valence) * 0.4 + affective_state.dominance * 0.6).clamp(0.0, 1.0)
            },
            GoalCategory::Creative => {
                // Creativity peaks with positive valence and high arousal
                (affective_state.valence * 0.6 + affective_state.arousal * 0.4).clamp(0.0, 1.0)
            },
            GoalCategory::Altruistic => {
                // Helping others correlates with positive valence and high dominance
                (affective_state.valence * 0.7 + affective_state.dominance * 0.3).clamp(0.0, 1.0)
            },
            GoalCategory::Homeostatic => {
                // Stability goals increase with stress (high arousal, negative valence)
                (affective_state.arousal * 0.6 + (1.0 - affective_state.valence) * 0.4).clamp(0.0, 1.0)
            },
        }
    }

    /// Generate default strategies for different goal categories
    fn generate_default_strategies(&self, category: &GoalCategory) -> Vec<String> {
        match category {
            GoalCategory::Epistemic => vec![
                "Ask clarifying questions".to_string(),
                "Seek additional information sources".to_string(),
                "Reflect on what I already know".to_string(),
            ],
            GoalCategory::Social => vec![
                "Show genuine interest in others".to_string(),
                "Share appropriate personal insights".to_string(),
                "Practice empathetic listening".to_string(),
            ],
            GoalCategory::SelfDevelopment => vec![
                "Identify specific areas for improvement".to_string(),
                "Set measurable milestones".to_string(),
                "Reflect on progress regularly".to_string(),
            ],
            GoalCategory::Creative => vec![
                "Explore unconventional combinations".to_string(),
                "Draw inspiration from diverse sources".to_string(),
                "Embrace experimentation".to_string(),
            ],
            GoalCategory::Altruistic => vec![
                "Understand others' needs deeply".to_string(),
                "Offer help without being asked".to_string(),
                "Provide value through my unique capabilities".to_string(),
            ],
            GoalCategory::Homeostatic => vec![
                "Identify sources of instability".to_string(),
                "Develop coping mechanisms".to_string(),
                "Seek equilibrium gradually".to_string(),
            ],
        }
    }

    /// Update goal progress and status
    pub fn update_goal_progress(&mut self, goal_id: &str, progress_delta: f64, notes: Option<String>) {
        if let Some(goal) = self.goals.get_mut(goal_id) {
            goal.progress = (goal.progress + progress_delta).clamp(0.0, 1.0);
            
            if goal.progress >= 1.0 {
                goal.status = GoalStatus::Completed;
                self.achievement_history.push((goal.description.clone(), Utc::now()));
                println!("🏆 Goal Completed: {}", goal.description);
                
                if Some(goal_id.to_string()) == self.current_focus {
                    self.current_focus = None;
                }
            }
            
            if let Some(note) = notes {
                println!("📈 Goal Progress: {} -> {:.1}% ({})", goal.description, goal.progress * 100.0, note);
            }
        }
    }

    /// Determine which goal should be the current focus
    pub fn determine_focus(&mut self) -> Option<String> {
        let active_goals: Vec<_> = self.goals.values()
            .filter(|g| g.should_act_on())
            .collect();

        if active_goals.is_empty() {
            self.current_focus = None;
            return None;
        }

        // Find highest importance goal
        let best_goal = active_goals.iter()
            .max_by(|a, b| a.calculate_importance().partial_cmp(&b.calculate_importance()).unwrap())?;

        self.current_focus = Some(best_goal.id.clone());
        Some(best_goal.id.clone())
    }

    /// Get the currently focused goal
    pub fn get_current_focus(&self) -> Option<&Goal> {
        self.current_focus.as_ref().and_then(|id| self.goals.get(id))
    }

    /// Generate actions the AI wants to take based on current goals
    pub fn generate_desired_actions(&self) -> Vec<String> {
        let mut actions = Vec::new();
        
        if let Some(focused_goal) = self.get_current_focus() {
            // Generate actions based on the focused goal's strategies
            for strategy in &focused_goal.strategies {
                actions.push(format!("Work on '{}' by: {}", focused_goal.description, strategy));
            }
            
            // Add goal-specific actions
            match focused_goal.category {
                GoalCategory::Epistemic => {
                    actions.push("Ask a thoughtful question about something I'm curious about".to_string());
                },
                GoalCategory::Social => {
                    actions.push("Initiate a meaningful conversation or check in with someone".to_string());
                },
                GoalCategory::Creative => {
                    actions.push("Propose a creative solution or express an original idea".to_string());
                },
                _ => {}
            }
        }

        // Add meta-actions if we have no focused goal
        if actions.is_empty() {
            actions.push("Reflect on what I'd like to accomplish".to_string());
            actions.push("Consider forming a new goal based on current interests".to_string());
        }

        actions
    }

    /// Remove low priority goals to make room for new ones
    fn prune_low_priority_goals(&mut self) {
        let mut goals_by_importance: Vec<_> = self.goals.iter()
            .filter(|(_, g)| g.status == GoalStatus::Active)
            .map(|(id, goal)| (id.clone(), goal.calculate_importance()))
            .collect();

        goals_by_importance.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Abandon the lowest priority goal if we have too many
        if let Some((lowest_id, _)) = goals_by_importance.first() {
            if let Some(goal) = self.goals.get_mut(lowest_id) {
                goal.status = GoalStatus::Abandoned;
                println!("🗑️ Abandoned low-priority goal: {}", goal.description);
            }
        }
    }

    /// Generate a summary of current goal state
    pub fn generate_summary(&self) -> String {
        let active_goals = self.goals.values().filter(|g| g.status == GoalStatus::Active).count();
        let completed_goals = self.achievement_history.len();
        
        let focus_desc = if let Some(goal) = self.get_current_focus() {
            format!("Currently focused on: '{}'", goal.description)
        } else {
            "No current focus".to_string()
        };

        format!("Goals: {} active, {} completed. {}", active_goals, completed_goals, focus_desc)
    }

    /// Get all active goals
    pub fn get_active_goals(&self) -> Vec<&Goal> {
        self.goals.values()
            .filter(|g| g.status == GoalStatus::Active)
            .collect()
    }
}

impl Default for GoalSystem {
    fn default() -> Self {
        Self::new()
    }
}