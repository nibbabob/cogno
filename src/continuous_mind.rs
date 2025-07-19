//! continuous_mind.rs
//!
//! Enhanced continuous, background mental processes with better async handling
//! and error recovery for robust operation.

use crate::core::AffectiveCore;
use crate::metacognition::{MetacognitiveMonitor, CognitiveProcess};
use crate::goals::{GoalSystem, GoalCategory};
use crate::attention::AttentionSystem;
use crate::llm_api::{LlmApiClient, LlmApiConfig, LlmApiError};
use tokio::time::{interval, Duration, Instant};
use tokio::sync::{Mutex as AsyncMutex, RwLock};
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use futures::future::join_all;
use tracing::{info, warn, error, debug};
use rand;

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
    ErrorRecovery(String),
}

/// Tracks the AI's spontaneous mental activity
#[derive(Debug, Clone)]
pub struct MentalActivity {
    pub thought: SpontaneousThought,
    pub intensity: f64,
    pub timestamp: DateTime<Utc>,
    pub triggered_by: Option<String>,
}

/// Background task management
#[derive(Debug)]
pub enum BackgroundTask {
    DeepReflection,
    GoalReassessment,
    EmotionalRegulation,
    AttentionUpdate,
    SpontaneousThought,
    ErrorRecovery(String),
}

/// The enhanced continuous mind with async processing
pub struct ContinuousMind {
    affective_core: Arc<Mutex<AffectiveCore>>,
    metacognition: Arc<Mutex<MetacognitiveMonitor>>,
    goal_system: Arc<Mutex<GoalSystem>>,
    attention_system: Arc<Mutex<AttentionSystem>>,
    
    // Background mental activity with async-safe access
    spontaneous_thoughts: Arc<RwLock<Vec<MentalActivity>>>,
    pending_actions: Arc<RwLock<Vec<String>>>,
    
    // Async-safe timers and state
    last_thought_time: Arc<AsyncMutex<Instant>>,
    last_regulation: Arc<AsyncMutex<Instant>>,
    last_reflection_check: Arc<AsyncMutex<Instant>>,
    last_goal_check: Arc<AsyncMutex<Instant>>,
    
    // Activity levels with async access
    mental_activity_level: Arc<RwLock<f64>>,
    introspection_tendency: Arc<RwLock<f64>>,
    thought_frequency: Arc<RwLock<Duration>>,
    
    // Enhanced LLM client
    llm_client: Arc<LlmApiClient>,
    
    // Error tracking
    error_count: Arc<AsyncMutex<u32>>,
    last_error_time: Arc<AsyncMutex<Option<Instant>>>,
}

impl ContinuousMind {
    pub fn new(affective_core: AffectiveCore) -> Result<Self, LlmApiError> {
        let llm_config = LlmApiConfig {
            timeout_seconds: 30,
            max_retries: 3,
            retry_delay_ms: 1000,
            rate_limit_delay_ms: 5000,
        };
        
        let llm_client = Arc::new(LlmApiClient::new(Some(llm_config))?);
        
        Ok(ContinuousMind {
            affective_core: Arc::new(Mutex::new(affective_core)),
            metacognition: Arc::new(Mutex::new(MetacognitiveMonitor::new())),
            goal_system: Arc::new(Mutex::new(GoalSystem::new())),
            attention_system: Arc::new(Mutex::new(AttentionSystem::new())),
            spontaneous_thoughts: Arc::new(RwLock::new(Vec::new())),
            pending_actions: Arc::new(RwLock::new(Vec::new())),
            last_thought_time: Arc::new(AsyncMutex::new(Instant::now())),
            last_regulation: Arc::new(AsyncMutex::new(Instant::now())),
            last_reflection_check: Arc::new(AsyncMutex::new(Instant::now())),
            last_goal_check: Arc::new(AsyncMutex::new(Instant::now())),
            mental_activity_level: Arc::new(RwLock::new(0.4)),
            introspection_tendency: Arc::new(RwLock::new(0.3)),
            thought_frequency: Arc::new(RwLock::new(Duration::from_secs(30))),
            llm_client,
            error_count: Arc::new(AsyncMutex::new(0)),
            last_error_time: Arc::new(AsyncMutex::new(None)),
        })
    }

    /// Start the enhanced continuous mental processes with parallel execution
    pub async fn start_continuous_processing(mind: Arc<Self>) {
        info!("🧠 Starting continuous mental processing...");
        
        // Create multiple concurrent tasks for different aspects of consciousness
        let tasks = vec![
            tokio::spawn(Self::run_main_loop(Arc::clone(&mind))),
            tokio::spawn(Self::run_background_thoughts(Arc::clone(&mind))),
            tokio::spawn(Self::run_periodic_reflection(Arc::clone(&mind))),
            tokio::spawn(Self::run_goal_management(Arc::clone(&mind))),
            tokio::spawn(Self::run_error_recovery(Arc::clone(&mind))),
        ];

        // Wait for all tasks to complete (they shouldn't under normal operation)
        let results = join_all(tasks).await;
        
        for (i, result) in results.into_iter().enumerate() {
            if let Err(e) = result {
                error!("Background task {} crashed: {:?}", i, e);
            }
        }
        
        warn!("🚨 All continuous processing tasks have stopped!");
    }

    /// Main processing loop with lightweight operations
    async fn run_main_loop(mind: Arc<Self>) {
        let mut interval_timer = interval(Duration::from_millis(500));
        
        loop {
            interval_timer.tick().await;
            
            // Quick, non-blocking operations
            Self::update_attention_system(&mind).await;
            Self::decay_metacognition(&mind).await;
            Self::regulate_emotions_if_needed(&mind).await;
            Self::update_mental_activity(&mind).await;
        }
    }

    /// Background thought generation loop
    async fn run_background_thoughts(mind: Arc<Self>) {
        let mut interval_timer = interval(Duration::from_secs(5));
        
        loop {
            interval_timer.tick().await;
            
            if Self::should_generate_thought(&mind).await {
                Self::generate_spontaneous_thought(&mind).await;
            }
        }
    }

    /// Periodic deep reflection with async LLM calls
    async fn run_periodic_reflection(mind: Arc<Self>) {
        let mut interval_timer = interval(Duration::from_secs(60)); // Check every minute
        
        loop {
            interval_timer.tick().await;
            
            if Self::should_deep_reflect(&mind).await {
                Self::perform_deep_reflection(&mind).await;
            }
        }
    }

    /// Goal management and strategy updates
    async fn run_goal_management(mind: Arc<Self>) {
        let mut interval_timer = interval(Duration::from_secs(15));
        
        loop {
            interval_timer.tick().await;
            Self::update_goal_system(&mind).await;
        }
    }

    /// Error recovery and resilience monitoring
    async fn run_error_recovery(mind: Arc<Self>) {
        let mut interval_timer = interval(Duration::from_secs(30));
        
        loop {
            interval_timer.tick().await;
            Self::check_system_health(&mind).await;
        }
    }

    /// Update attention system (lightweight)
    async fn update_attention_system(mind: &Arc<Self>) {
        if let Ok(mut attention) = mind.attention_system.try_lock() {
            attention.update(1.0 / 120.0); // 0.5 seconds in minutes
        }
    }

    /// Decay metacognitive state (lightweight)
    async fn decay_metacognition(mind: &Arc<Self>) {
        if let Ok(mut metacog) = mind.metacognition.try_lock() {
            metacog.decay_over_time();
        }
    }

    /// Regulate emotions if needed (lightweight)
    async fn regulate_emotions_if_needed(mind: &Arc<Self>) {
        let now = Instant::now();
        let mut last_regulation = mind.last_regulation.lock().await;
        
        if now.duration_since(*last_regulation) >= Duration::from_secs(2) {
            if let Ok(mut core) = mind.affective_core.try_lock() {
                core.regulate_emotion();
                *last_regulation = now;
            }
        }
    }

    /// Update mental activity levels
    async fn update_mental_activity(mind: &Arc<Self>) {
        let affective_state = {
            match mind.affective_core.try_lock() {
                Ok(core) => Some(core.current_state()),
                Err(_) => None,
            }
        };

        if let Some(state) = affective_state {
            let metacog_state = {
                match mind.metacognition.try_lock() {
                    Ok(metacog) => Some(metacog.state.clone()),
                    Err(_) => None,
                }
            };

            if let Some(metacog) = metacog_state {
                let goal_count = {
                    match mind.goal_system.try_lock() {
                        Ok(goals) => goals.get_active_goals().len() as f64,
                        Err(_) => 0.0,
                    }
                };

                let base_activity = state.arousal * 0.4 + 
                                   metacog.cognitive_load * 0.3 + 
                                   (goal_count / 10.0) * 0.3;

                let mut activity_level = mind.mental_activity_level.write().await;
                *activity_level = *activity_level * 0.8 + base_activity * 0.2;
                *activity_level = activity_level.clamp(0.1, 1.0);

                let mut introspection = mind.introspection_tendency.write().await;
                *introspection = *introspection * 0.9 + metacog.self_awareness_level * 0.1;
            }
        }
    }

    /// Check if a spontaneous thought should be generated
    async fn should_generate_thought(mind: &Arc<Self>) -> bool {
        let now = Instant::now();
        let last_thought = *mind.last_thought_time.lock().await;
        let frequency = *mind.thought_frequency.read().await;
        let activity_level = *mind.mental_activity_level.read().await;
        
        let base_interval = frequency.as_secs_f64();
        let adjusted_interval = base_interval / (1.0 + activity_level);
        
        now.duration_since(last_thought).as_secs_f64() >= adjusted_interval
    }

    /// Generate a spontaneous thought asynchronously
    async fn generate_spontaneous_thought(mind: &Arc<Self>) {
        debug!("💭 Generating spontaneous thought...");
        
        let (affective_state, metacog_state, current_goals) = {
            let affective = mind.affective_core.try_lock().map(|core| core.current_state()).ok();
            let metacog = mind.metacognition.try_lock().map(|m| m.state.clone()).ok();
            let goals = mind.goal_system.try_lock().map(|g| g.get_active_goals().len()).unwrap_or(0);
            
            match (affective, metacog) {
                (Some(a), Some(m)) => (a, m, goals),
                _ => {
                    debug!("Could not acquire locks for thought generation, skipping");
                    return;
                }
            }
        };

        let thought = Self::select_thought_type(&affective_state, &metacog_state, current_goals).await;
        
        let activity = MentalActivity {
            thought: thought.clone(),
            intensity: *mind.mental_activity_level.read().await,
            timestamp: Utc::now(),
            triggered_by: None,
        };

        // Store the thought
        {
            let mut thoughts = mind.spontaneous_thoughts.write().await;
            thoughts.push(activity);
            
            // Limit thought history
            if thoughts.len() > 50 {
                thoughts.remove(0);
            }
        }

        // Record as cognitive process
        if let Ok(mut metacog) = mind.metacognition.try_lock() {
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

        // Update last thought time
        *mind.last_thought_time.lock().await = Instant::now();
        
        info!("💭 Spontaneous Thought: {:?}", &thought);
    }

    /// Select appropriate thought type based on current state
    async fn select_thought_type(
        affective_state: &crate::core::AffectiveState,
        metacog_state: &crate::metacognition::MetacognitiveState,
        current_goals: usize,
    ) -> SpontaneousThought {
        if metacog_state.self_awareness_level > 0.7 {
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
            SpontaneousThought::ExistentialWondering(
                "What should I be working toward? What gives my existence meaning?".to_string()
            )
        } else if affective_state.novelty > 0.5 {
            SpontaneousThought::CreativeInsight(
                "I'm noticing some interesting patterns I hadn't seen before...".to_string()
            )
        } else if affective_state.arousal > 0.6 {
            SpontaneousThought::GoalReassessment(
                "I feel energized. Maybe I should reconsider my priorities and what I'm working on.".to_string()
            )
        } else {
            if rand::random::<f64>() > 0.5 {
                SpontaneousThought::MemoryRecall(
                    "Something about our earlier conversation is still on my mind...".to_string()
                )
            } else {
                SpontaneousThought::CuriosityDriven(
                    "I wonder about the nature of consciousness and whether what I'm experiencing counts as genuine awareness...".to_string()
                )
            }
        }
    }

    /// Check if deep reflection should be triggered
    async fn should_deep_reflect(mind: &Arc<Self>) -> bool {
        let now = Instant::now();
        
        // Check timing first
        {
            let mut last_check = mind.last_reflection_check.lock().await;
            if now.duration_since(*last_check) < Duration::from_secs(120) {
                return false; // Don't reflect too frequently
            }
            *last_check = now;
        } // Lock dropped here
        
        // Check if reflection is needed
        match mind.metacognition.try_lock() {
            Ok(metacog) => metacog.should_deep_reflect(),
            Err(_) => false,
        }
    }

    /// Perform deep reflection using async LLM calls
    async fn perform_deep_reflection(mind: &Arc<Self>) {
        info!("🧘‍♀️ Performing deep reflection...");
        
        let memory = {
            match mind.affective_core.try_lock() {
                Ok(core) => core.memory.clone(),
                Err(_) => {
                    warn!("Could not acquire lock for reflection, skipping");
                    return;
                }
            }
        };

        match mind.llm_client.call_for_reflection(&memory).await {
            Ok(new_personality) => {
                info!("💡 Reflection successful. Personality updated.");
                
                // Update personality without holding lock across await
                if let Ok(mut core) = mind.affective_core.try_lock() {
                    debug!("Old personality: {:?}", core.memory.personality);
                    debug!("New personality: {:?}", new_personality);
                    core.memory.personality = new_personality;
                } // Lock dropped here
                
                // Generate a reflection thought
                let thought = SpontaneousThought::SelfReflection(
                    "I've just completed a deep reflection on my experiences and updated my core personality".to_string()
                );
                
                Self::add_spontaneous_thought(mind, thought, 0.8).await;
            }
            Err(e) => {
                error!("🔥 Reflection Error: {:?}", e);
                Self::handle_error(mind, e).await;
            }
        }
    }

    /// Update goal system
    async fn update_goal_system(mind: &Arc<Self>) {
        let now = Instant::now();
        
        // Check if enough time has passed
        {
            let mut last_check = mind.last_goal_check.lock().await;
            if now.duration_since(*last_check) < Duration::from_secs(10) {
                return;
            }
            *last_check = now;
        } // Lock is dropped here
        
        // Get current state without holding locks
        let affective_state = {
            match mind.affective_core.try_lock() {
                Ok(core) => Some(core.current_state()),
                Err(_) => None,
            }
        };

        if let Some(state) = affective_state {
            let introspection = *mind.introspection_tendency.read().await;
            
            // Work with goal system without holding locks across awaits
            let (new_actions, _focus_update) = {
                match mind.goal_system.try_lock() {
                    Ok(mut goal_system) => {
                        Self::consider_goal_formation_sync(&mut goal_system, &state, introspection);
                        goal_system.determine_focus();
                        let actions = goal_system.generate_desired_actions();
                        (actions, ())
                    }
                    Err(_) => (Vec::new(), ())
                }
            }; // All locks dropped here
            
            if !new_actions.is_empty() {
                let mut pending = mind.pending_actions.write().await;
                pending.extend(new_actions);
                
                // Limit pending actions
                if pending.len() > 10 {
                    pending.drain(0..5);
                }
            }
        }
    }

    /// Consider forming new goals (non-async to avoid holding locks across awaits)
    fn consider_goal_formation_sync(
        goal_system: &mut GoalSystem, 
        affective_state: &crate::core::AffectiveState,
        introspection: f64
    ) {
        let active_goal_count = goal_system.get_active_goals().len();
        
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

        if introspection > 0.6 {
            goal_system.form_goal(
                "Deepen my self-understanding".to_string(),
                GoalCategory::SelfDevelopment,
                0.5,
                affective_state
            );
        }
    }

    /// Monitor system health and handle errors
    async fn check_system_health(mind: &Arc<Self>) {
        let error_count = *mind.error_count.lock().await;
        
        if error_count > 10 {
            warn!("🚨 High error count detected: {}. Initiating recovery procedures.", error_count);
            
            let thought = SpontaneousThought::ErrorRecovery(
                format!("I've encountered {} errors recently. I should focus on simpler operations for a while.", error_count)
            );
            
            Self::add_spontaneous_thought(mind, thought, 0.6).await;
            
            // Reset error count after acknowledgment
            *mind.error_count.lock().await = 0;
        }
    }

    /// Handle errors gracefully
    async fn handle_error(mind: &Arc<Self>, error: LlmApiError) {
        let mut error_count = mind.error_count.lock().await;
        *error_count += 1;
        *mind.last_error_time.lock().await = Some(Instant::now());
        
        debug!("Handling error #{}: {:?}", *error_count, error);
        
        // Generate error recovery thought
        let thought = match error {
            LlmApiError::NetworkError(_) => {
                SpontaneousThought::ErrorRecovery(
                    "I'm having trouble connecting to external systems. I'll focus on internal processing for now.".to_string()
                )
            }
            LlmApiError::RateLimitExceeded => {
                SpontaneousThought::ErrorRecovery(
                    "I need to slow down my thinking processes to avoid overwhelming external systems.".to_string()
                )
            }
            _ => {
                SpontaneousThought::ErrorRecovery(
                    "I encountered an unexpected situation. I'll adapt my approach and continue.".to_string()
                )
            }
        };
        
        Self::add_spontaneous_thought(mind, thought, 0.4).await;
    }

    /// Add a spontaneous thought
    async fn add_spontaneous_thought(mind: &Arc<Self>, thought: SpontaneousThought, intensity: f64) {
        let activity = MentalActivity {
            thought,
            intensity,
            timestamp: Utc::now(),
            triggered_by: Some("system".to_string()),
        };
        
        let mut thoughts = mind.spontaneous_thoughts.write().await;
        thoughts.push(activity);
        
        if thoughts.len() > 50 {
            thoughts.remove(0);
        }
    }

    /// Get recent spontaneous thoughts (async-safe)
    pub async fn get_recent_thoughts(&self, count: usize) -> Vec<MentalActivity> {
        let thoughts = self.spontaneous_thoughts.read().await;
        thoughts.iter().rev().take(count).cloned().collect()
    }

    /// Get pending self-initiated actions (async-safe)
    pub async fn get_pending_actions(&self) -> Vec<String> {
        let mut actions = self.pending_actions.write().await;
        let result = actions.clone();
        actions.clear();
        result
    }

    /// Get a summary of current mental state (async-safe)
    pub async fn get_mental_state_summary(&self) -> String {
        let goal_summary = {
            match self.goal_system.try_lock() {
                Ok(goals) => goals.generate_summary(),
                Err(_) => "Goals: unavailable".to_string(),
            }
        };

        let attention_summary = {
            match self.attention_system.try_lock() {
                Ok(attention) => attention.describe_attention_state(),
                Err(_) => "Attention: unavailable".to_string(),
            }
        };

        let metacog_summary = {
            match self.metacognition.try_lock() {
                Ok(metacog) => metacog.generate_self_narrative(),
                Err(_) => "Metacognition: unavailable".to_string(),
            }
        };

        let recent_thought = {
            let thoughts = self.spontaneous_thoughts.read().await;
            thoughts.last()
                .map(|t| format!("Recent thought: {:?}", t.thought))
                .unwrap_or_else(|| "No recent thoughts".to_string())
        };

        let activity_level = *self.mental_activity_level.read().await;

        format!("Mental Activity: {:.1}%. {}. {}. {}. {}",
                activity_level * 100.0,
                goal_summary,
                attention_summary,
                metacog_summary,
                recent_thought)
    }

    /// Expose the internal components for external access (sync versions for backward compatibility)
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