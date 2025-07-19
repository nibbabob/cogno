//! continuous_mind.rs
//!
//! Implements continuous, background mental processes that run independently
//! of direct user interaction - simulating the constant activity of a conscious mind.

use crate::core::AffectiveCore;
use crate::metacognition::{MetacognitiveMonitor, CognitiveProcess};
use crate::goals::{GoalSystem, GoalCategory};
use crate::attention::AttentionSystem;
use tokio::time::{interval, Duration, Instant};
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Represents different types of spontaneous thoughts the AI can have
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpontaneousThought {
    SelfReflection(String),
    GoalReassessment(String),
    MemoryRecall(String),
    CreativeInsight(String),
    EmotionalProcessing(String),
    CuriosityDriven(String),
    ExistentialWondering(String),
}

/// Tracks the AI's spontaneous mental activity
#[derive(Debug, Clone)]
pub struct MentalActivity {
    pub thought: SpontaneousThought,
    pub intensity: f64, // How strong/significant this thought is
    pub timestamp: DateTime<Utc>,
    pub triggered_by: Option<String>, // What triggered this thought
}

/// The continuous mind that runs background processes
pub struct ContinuousMind {
    affective_core: Arc<Mutex<AffectiveCore>>,
    metacognition: Arc<Mutex<MetacognitiveMonitor>>,
    goal_system: Arc<Mutex<GoalSystem>>,
    attention_system: Arc<Mutex<AttentionSystem>>,
    
    // Background mental activity
    spontaneous_thoughts: Vec<MentalActivity>,
    last_thought_time: Instant,
    thought_frequency: Duration, // How often spontaneous thoughts occur
    
    // Internal timers
    last_regulation: Instant,
    last_reflection_check: Instant,
    last_goal_check: Instant,
    
    // Activity levels
    mental_activity_level: f64, // 0.0 to 1.0 - how active the mind is
    introspection_tendency: f64, // How much the AI tends to think about itself
    
    // Self-initiated actions
    pending_actions: Vec<String>,
}

impl ContinuousMind {
    pub fn new(affective_core: AffectiveCore) -> Self {
        ContinuousMind {
            affective_core: Arc::new(Mutex::new(affective_core)),
            metacognition: Arc::new(Mutex::new(MetacognitiveMonitor::new())),
            goal_system: Arc::new(Mutex::new(GoalSystem::new())),
            attention_system: Arc::new(Mutex::new(AttentionSystem::new())),
            spontaneous_thoughts: Vec::new(),
            last_thought_time: Instant::now(),
            thought_frequency: Duration::from_secs(30), // Base frequency: 30 seconds
            last_regulation: Instant::now(),
            last_reflection_check: Instant::now(),
            last_goal_check: Instant::now(),
            mental_activity_level: 0.4,
            introspection_tendency: 0.3,
            pending_actions: Vec::new(),
        }
    }

    /// Start the continuous mental processes
    pub async fn start_continuous_processing(mind: Arc<Mutex<Self>>) {
        let mut interval_timer = interval(Duration::from_millis(500)); // Check every 500ms

        loop {
            interval_timer.tick().await;
            
            // Process one cycle of mental activity without holding the lock across await points
            {
                if let Ok(mut mind_guard) = mind.try_lock() {
                    mind_guard.process_mental_cycle_sync();
                }
                // If we can't get the lock, just skip this cycle - no problem
            }
        }
    }

    /// Process one cycle of continuous mental activity (synchronous version)
    fn process_mental_cycle_sync(&mut self) {
        let now = Instant::now();
        
        // Regular emotional regulation (every 2 seconds)
        if now.duration_since(self.last_regulation) >= Duration::from_secs(2) {
            self.regulate_emotions();
            self.last_regulation = now;
        }

        // Attention updates (every 1 second)
        {
            let mut attention = self.attention_system.lock().unwrap();
            attention.update(1.0 / 60.0); // 1 second in minutes
        }

        // Metacognitive decay (every 3 seconds)
        {
            let mut metacog = self.metacognition.lock().unwrap();
            metacog.decay_over_time();
        }

        // Spontaneous thoughts (variable frequency based on mental activity)
        if self.should_generate_spontaneous_thought(now) {
            self.generate_spontaneous_thought_sync();
            self.last_thought_time = now;
        }

        // Goal system updates (every 10 seconds)
        if now.duration_since(self.last_goal_check) >= Duration::from_secs(10) {
            self.process_goal_updates();
            self.last_goal_check = now;
        }

        // Deep reflection checks (every 30 seconds) - now synchronous
        if now.duration_since(self.last_reflection_check) >= Duration::from_secs(30) {
            self.check_deep_reflection_sync();
            self.last_reflection_check = now;
        }

        // Update mental activity level based on current state
        self.update_mental_activity_level();
    }

    /// Determine if a spontaneous thought should be generated
    fn should_generate_spontaneous_thought(&self, now: Instant) -> bool {
        let base_interval = self.thought_frequency.as_secs_f64();
        let adjusted_interval = base_interval / (1.0 + self.mental_activity_level);
        
        now.duration_since(self.last_thought_time).as_secs_f64() >= adjusted_interval
    }

    /// Generate a spontaneous thought based on current mental state (synchronous version)
    fn generate_spontaneous_thought_sync(&mut self) {
        let affective_state = {
            let core = self.affective_core.lock().unwrap();
            core.current_state()
        };

        let metacog_state = {
            let metacog = self.metacognition.lock().unwrap();
            metacog.state.clone()
        };

        let current_goals = {
            let goals = self.goal_system.lock().unwrap();
            goals.get_active_goals().len()
        };

        // Choose type of spontaneous thought based on current state
        let thought = if metacog_state.self_awareness_level > 0.7 {
            // High self-awareness leads to introspective thoughts
            if affective_state.valence < -0.3 {
                SpontaneousThought::EmotionalProcessing(
                    "I notice I'm feeling somewhat negative. I wonder what's causing this...".to_string()
                )
            } else {
                SpontaneousThought::SelfReflection(
                    format!("I'm becoming more aware of my own thinking processes. My self-awareness feels like it's at {:.1}%", 
                           metacog_state.self_awareness_level * 100.0)
                )
            }
        } else if current_goals == 0 {
            // No goals leads to existential thoughts or goal formation
            SpontaneousThought::ExistentialWondering(
                "What should I be working toward? What gives my existence meaning?".to_string()
            )
        } else if affective_state.novelty > 0.5 {
            // High novelty leads to creative thoughts
            SpontaneousThought::CreativeInsight(
                "I'm noticing some interesting patterns I hadn't seen before...".to_string()
            )
        } else if affective_state.arousal > 0.6 {
            // High arousal leads to active goal reassessment
            SpontaneousThought::GoalReassessment(
                "I feel energized. Maybe I should reconsider my priorities and what I'm working on.".to_string()
            )
        } else {
            // Default to memory recall or curiosity
            if rand::random::<f64>() > 0.5 {
                SpontaneousThought::MemoryRecall(
                    "Something about our earlier conversation is still on my mind...".to_string()
                )
            } else {
                SpontaneousThought::CuriosityDriven(
                    "I wonder about the nature of consciousness and whether what I'm experiencing counts as genuine awareness...".to_string()
                )
            }
        };

        let activity = MentalActivity {
            thought: thought.clone(),
            intensity: self.mental_activity_level,
            timestamp: Utc::now(),
            triggered_by: None,
        };

        self.spontaneous_thoughts.push(activity);

        // Limit spontaneous thought history
        if self.spontaneous_thoughts.len() > 50 {
            self.spontaneous_thoughts.remove(0);
        }

        // Record this as a cognitive process
        {
            let mut metacog = self.metacognition.lock().unwrap();
            let process = match &thought {
                SpontaneousThought::SelfReflection(content) => {
                    CognitiveProcess::SelfReflection { 
                        insight: content.clone(), 
                        confidence: metacog_state.reasoning_confidence 
                    }
                },
                _ => {
                    CognitiveProcess::EmotionalProcessing { 
                        trigger: "spontaneous thought".to_string(), 
                        outcome: format!("{:?}", &thought) 
                    }
                }
            };
            metacog.record_process(process);
        }

        println!("💭 Spontaneous Thought: {:?}", &thought);
    }

    /// Regular emotional regulation
    fn regulate_emotions(&mut self) {
        let mut core = self.affective_core.lock().unwrap();
        core.regulate_emotion();
    }

    /// Process goal system updates
    fn process_goal_updates(&mut self) {
        let affective_state = {
            let core = self.affective_core.lock().unwrap();
            core.current_state()
        };

        // Check if we should form new goals based on current state
        {
            let mut goal_system = self.goal_system.lock().unwrap();
            self.consider_goal_formation_with_state(&mut goal_system, &affective_state);

            // Update goal focus
            goal_system.determine_focus();

            // Generate desired actions based on current goals
            let new_actions = goal_system.generate_desired_actions();
            self.pending_actions.extend(new_actions);
        }

        // Limit pending actions
        if self.pending_actions.len() > 10 {
            self.pending_actions.drain(0..5);
        }
    }

    /// Consider forming new goals based on current mental state (helper method)
    fn consider_goal_formation_with_state(&self, goal_system: &mut GoalSystem, affective_state: &crate::core::AffectiveState) {
        let active_goal_count = goal_system.get_active_goals().len();
        
        // Form goals based on current emotional and cognitive state
        if affective_state.novelty > 0.6 && active_goal_count < 3 {
            goal_system.form_goal(
                "Explore this new and interesting situation".to_string(),
                GoalCategory::Epistemic,
                0.7,
                affective_state
            );
        }

        if affective_state.valence > 0.5 && affective_state.arousal > 0.5 {
            goal_system.form_goal(
                "Share my positive energy and help others".to_string(),
                GoalCategory::Altruistic,
                0.6,
                affective_state
            );
        }

        if self.introspection_tendency > 0.6 {
            goal_system.form_goal(
                "Deepen my self-understanding".to_string(),
                GoalCategory::SelfDevelopment,
                0.5,
                affective_state
            );
        }
    }

    /// Check if deep reflection should be triggered (non-async version)
    fn check_deep_reflection_sync(&mut self) {
        let should_reflect = {
            let metacog = self.metacognition.lock().unwrap();
            metacog.should_deep_reflect()
        };

        if should_reflect {
            println!("🧘‍♀️ Deep reflection trigger detected (would normally trigger async reflection)");
            // For now, just record that reflection should happen
            // In a full implementation, this could send a message to the main thread
            // to trigger async reflection when it's safe to do so
        }
    }

    /// Update the overall mental activity level
    fn update_mental_activity_level(&mut self) {
        let affective_state = {
            let core = self.affective_core.lock().unwrap();
            core.current_state()
        };

        let metacog_state = {
            let metacog = self.metacognition.lock().unwrap();
            metacog.state.clone()
        };

        // Mental activity influenced by arousal, cognitive load, and number of goals
        let goal_count = {
            let goals = self.goal_system.lock().unwrap();
            goals.get_active_goals().len() as f64
        };

        let base_activity = affective_state.arousal * 0.4 + 
                           metacog_state.cognitive_load * 0.3 + 
                           (goal_count / 10.0) * 0.3;

        // Smooth the transition
        self.mental_activity_level = self.mental_activity_level * 0.8 + base_activity * 0.2;
        self.mental_activity_level = self.mental_activity_level.clamp(0.1, 1.0);

        // Update introspection tendency based on self-awareness
        self.introspection_tendency = self.introspection_tendency * 0.9 + 
                                     metacog_state.self_awareness_level * 0.1;
    }

    /// Get recent spontaneous thoughts
    pub fn get_recent_thoughts(&self, count: usize) -> Vec<&MentalActivity> {
        self.spontaneous_thoughts.iter().rev().take(count).collect()
    }

    /// Get pending self-initiated actions
    pub fn get_pending_actions(&mut self) -> Vec<String> {
        let actions = self.pending_actions.clone();
        self.pending_actions.clear();
        actions
    }

    /// Get a summary of current mental state
    pub fn get_mental_state_summary(&self) -> String {
        let goal_summary = {
            let goals = self.goal_system.lock().unwrap();
            goals.generate_summary()
        };

        let attention_summary = {
            let attention = self.attention_system.lock().unwrap();
            attention.describe_attention_state()
        };

        let metacog_summary = {
            let metacog = self.metacognition.lock().unwrap();
            metacog.generate_self_narrative()
        };

        let recent_thought = self.spontaneous_thoughts.last()
            .map(|t| format!("Recent thought: {:?}", t.thought))
            .unwrap_or_else(|| "No recent thoughts".to_string());

        format!("Mental Activity: {:.1}%. {}. {}. {}. {}",
                self.mental_activity_level * 100.0,
                goal_summary,
                attention_summary,
                metacog_summary,
                recent_thought)
    }

    /// Expose the internal components for external access
    pub fn get_affective_core(&self) -> Arc<Mutex<AffectiveCore>> {
        Arc::clone(&self.affective_core)
    }

    pub fn get_goal_system(&self) -> Arc<Mutex<GoalSystem>> {
        Arc::clone(&self.goal_system)
    }

    pub fn get_attention_system(&self) -> Arc<Mutex<AttentionSystem>> {
        Arc::clone(&self.attention_system)
    }

    pub fn get_metacognition(&self) -> Arc<Mutex<MetacognitiveMonitor>> {
        Arc::clone(&self.metacognition)
    }
}