//! continuous_mind.rs
//!
//! Enhanced continuous, background mental processes with complete feature integration

use crate::core::AffectiveCore;
use crate::metacognition::{MetacognitiveMonitor, CognitiveProcess};
use crate::goals::GoalSystem;
use crate::attention::{AttentionSystem, AttentionTarget};
use crate::llm_api::{LlmApiClient, LlmApiConfig, LlmApiError};
use tokio::time::{interval, Duration, Instant};
use tokio::sync::{Mutex as AsyncMutex, RwLock};
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use futures::future::join_all;
use tracing::{info, warn, error, debug};

/// Simple random selection helper that avoids trait bound issues
fn simple_random_choice<T: Clone>(choices: &[T]) -> T {
    let random_val = (rand::random::<f64>() * 1000.0) as usize;
    let index = random_val % choices.len();
    choices[index].clone()
}

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
    AttentionShift(String),
    SystemIntegration(String),
}

/// Tracks the AI's spontaneous mental activity with full field utilization
#[derive(Debug, Clone)]
pub struct MentalActivity {
    pub thought: SpontaneousThought,
    pub intensity: f64,         // Now actively used for prioritization
    pub timestamp: DateTime<Utc>, // Now used for temporal analysis
    pub triggered_by: Option<String>, // Now used for causal tracking
}

impl MentalActivity {
    /// Calculate how recent this mental activity is (0.0 = very old, 1.0 = just now)
    pub fn recency_score(&self) -> f64 {
        let now = Utc::now();
        let age = now.signed_duration_since(self.timestamp);
        let age_minutes = age.num_minutes() as f64;
        
        // Activities become less relevant after 30 minutes
        (1.0 - (age_minutes / 30.0)).max(0.0)
    }
    
    /// Calculate overall relevance score combining intensity and recency
    pub fn relevance_score(&self) -> f64 {
        (self.intensity * 0.7) + (self.recency_score() * 0.3)
    }
    
    /// Check if this activity should trigger follow-up processing
    pub fn needs_follow_up(&self) -> bool {
        self.intensity > 0.7 && self.recency_score() > 0.5
    }
}

/// Enhanced background task management with full utilization
#[derive(Debug, Clone)]
pub enum BackgroundTask {
    DeepReflection,
    GoalReassessment,
    EmotionalRegulation,
    AttentionUpdate,
    SpontaneousThought,
    ErrorRecovery(String),
    MemoryConsolidation,
    SystemHealthCheck,
    CreativeIncubation,
    SocialContextAnalysis,
}

impl BackgroundTask {
    /// Get the priority of this task (higher = more important)
    pub fn priority(&self) -> f64 {
        match self {
            BackgroundTask::ErrorRecovery(_) => 1.0,
            BackgroundTask::EmotionalRegulation => 0.9,
            BackgroundTask::DeepReflection => 0.8,
            BackgroundTask::SystemHealthCheck => 0.7,
            BackgroundTask::GoalReassessment => 0.6,
            BackgroundTask::AttentionUpdate => 0.5,
            BackgroundTask::MemoryConsolidation => 0.4,
            BackgroundTask::SpontaneousThought => 0.3,
            BackgroundTask::CreativeIncubation => 0.3,
            BackgroundTask::SocialContextAnalysis => 0.2,
        }
    }
    
    /// Get expected execution time in seconds
    pub fn execution_time(&self) -> u64 {
        match self {
            BackgroundTask::DeepReflection => 60,
            BackgroundTask::MemoryConsolidation => 30,
            BackgroundTask::GoalReassessment => 15,
            BackgroundTask::ErrorRecovery(_) => 10,
            BackgroundTask::SystemHealthCheck => 5,
            BackgroundTask::EmotionalRegulation => 2,
            BackgroundTask::AttentionUpdate => 1,
            BackgroundTask::SpontaneousThought => 3,
            BackgroundTask::CreativeIncubation => 20,
            BackgroundTask::SocialContextAnalysis => 10,
        }
    }
}

/// Task scheduler for managing background operations
#[derive(Debug)]
pub struct TaskScheduler {
    pending_tasks: Vec<(BackgroundTask, Instant)>,
    running_tasks: Vec<(BackgroundTask, Instant)>,
    completed_tasks: Vec<(BackgroundTask, Instant)>,
    max_concurrent: usize,
}

impl TaskScheduler {
    pub fn new() -> Self {
        TaskScheduler {
            pending_tasks: Vec::new(),
            running_tasks: Vec::new(),
            completed_tasks: Vec::new(),
            max_concurrent: 3,
        }
    }
    
    pub fn schedule_task(&mut self, task: BackgroundTask) {
        self.pending_tasks.push((task, Instant::now()));
        // Sort by priority
        self.pending_tasks.sort_by(|a, b| b.0.priority().partial_cmp(&a.0.priority()).unwrap());
    }
    
    pub fn get_next_task(&mut self) -> Option<BackgroundTask> {
        if self.running_tasks.len() < self.max_concurrent && !self.pending_tasks.is_empty() {
            let (task, start_time) = self.pending_tasks.remove(0);
            self.running_tasks.push((task.clone(), start_time));
            Some(task)
        } else {
            None
        }
    }
    
    pub fn complete_task(&mut self, task: &BackgroundTask) {
        if let Some(pos) = self.running_tasks.iter().position(|(t, _)| {
            std::mem::discriminant(t) == std::mem::discriminant(task)
        }) {
            let completed = self.running_tasks.remove(pos);
            self.completed_tasks.push(completed);
            
            // Keep only recent completed tasks
            if self.completed_tasks.len() > 50 {
                self.completed_tasks.remove(0);
            }
        }
    }
    
    pub fn get_status(&self) -> String {
        format!("Tasks - Pending: {}, Running: {}, Completed: {}", 
                self.pending_tasks.len(), 
                self.running_tasks.len(), 
                self.completed_tasks.len())
    }
}

/// The enhanced continuous mind with complete feature integration
pub struct ContinuousMind {
    affective_core: Arc<Mutex<AffectiveCore>>,
    metacognition: Arc<Mutex<MetacognitiveMonitor>>,
    goal_system: Arc<Mutex<GoalSystem>>,
    attention_system: Arc<Mutex<AttentionSystem>>,
    
    // Enhanced mental activity tracking with full utilization
    spontaneous_thoughts: Arc<RwLock<Vec<MentalActivity>>>,
    pending_actions: Arc<RwLock<Vec<String>>>,
    
    // Task management system
    task_scheduler: Arc<AsyncMutex<TaskScheduler>>,
    
    // Async-safe timers and state
    last_thought_time: Arc<AsyncMutex<Instant>>,
    last_regulation: Arc<AsyncMutex<Instant>>,
    last_reflection_check: Arc<AsyncMutex<Instant>>,
    last_goal_check: Arc<AsyncMutex<Instant>>,
    last_memory_consolidation: Arc<AsyncMutex<Instant>>,
    
    // Enhanced activity levels with full utilization
    mental_activity_level: Arc<RwLock<f64>>,
    introspection_tendency: Arc<RwLock<f64>>,
    thought_frequency: Arc<RwLock<Duration>>,
    creativity_level: Arc<RwLock<f64>>,
    social_awareness: Arc<RwLock<f64>>,
    
    // Enhanced LLM client
    llm_client: Arc<LlmApiClient>,
    
    // Comprehensive error tracking
    error_count: Arc<AsyncMutex<u32>>,
    last_error_time: Arc<AsyncMutex<Option<Instant>>>,
    error_types: Arc<RwLock<Vec<String>>>,
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
            task_scheduler: Arc::new(AsyncMutex::new(TaskScheduler::new())),
            last_thought_time: Arc::new(AsyncMutex::new(Instant::now())),
            last_regulation: Arc::new(AsyncMutex::new(Instant::now())),
            last_reflection_check: Arc::new(AsyncMutex::new(Instant::now())),
            last_goal_check: Arc::new(AsyncMutex::new(Instant::now())),
            last_memory_consolidation: Arc::new(AsyncMutex::new(Instant::now())),
            mental_activity_level: Arc::new(RwLock::new(0.4)),
            introspection_tendency: Arc::new(RwLock::new(0.3)),
            thought_frequency: Arc::new(RwLock::new(Duration::from_secs(30))),
            creativity_level: Arc::new(RwLock::new(0.5)),
            social_awareness: Arc::new(RwLock::new(0.4)),
            llm_client,
            error_count: Arc::new(AsyncMutex::new(0)),
            last_error_time: Arc::new(AsyncMutex::new(None)),
            error_types: Arc::new(RwLock::new(Vec::new())),
        })
    }

    /// Start the enhanced continuous mental processes with full task management
    pub async fn start_continuous_processing(mind: Arc<Self>) {
        info!("🧠 Starting enhanced continuous mental processing with full task scheduling...");
        
        // Create comprehensive concurrent tasks
        let tasks = vec![
            tokio::spawn(Self::run_main_loop(Arc::clone(&mind))),
            tokio::spawn(Self::run_background_thoughts(Arc::clone(&mind))),
            tokio::spawn(Self::run_task_scheduler(Arc::clone(&mind))),
            tokio::spawn(Self::run_memory_consolidation(Arc::clone(&mind))),
            tokio::spawn(Self::run_creative_incubation(Arc::clone(&mind))),
            tokio::spawn(Self::run_social_context_analysis(Arc::clone(&mind))),
            tokio::spawn(Self::run_system_monitoring(Arc::clone(&mind))),
        ];

        let results = join_all(tasks).await;
        
        for (i, result) in results.into_iter().enumerate() {
            if let Err(e) = result {
                error!("Background task {} crashed: {:?}", i, e);
            }
        }
        
        warn!("🚨 All enhanced continuous processing tasks have stopped!");
    }

    /// Enhanced main processing loop
    async fn run_main_loop(mind: Arc<Self>) {
        let mut interval_timer = interval(Duration::from_millis(500));
        
        loop {
            interval_timer.tick().await;
            
            Self::update_attention_system(&mind).await;
            Self::decay_metacognition(&mind).await;
            Self::regulate_emotions_if_needed(&mind).await;
            Self::update_comprehensive_mental_state(&mind).await;
            Self::process_pending_thoughts(&mind).await;
        }
    }

    /// Enhanced background thought generation with full utilization
    async fn run_background_thoughts(mind: Arc<Self>) {
        let mut interval_timer = interval(Duration::from_secs(3));
        
        loop {
            interval_timer.tick().await;
            
            if Self::should_generate_thought(&mind).await {
                Self::generate_enhanced_spontaneous_thought(&mind).await;
            }
            
            Self::analyze_thought_patterns(&mind).await;
        }
    }

    /// New: Task scheduler runner
    async fn run_task_scheduler(mind: Arc<Self>) {
        let mut interval_timer = interval(Duration::from_secs(5));
        
        loop {
            interval_timer.tick().await;
            Self::process_scheduled_tasks(&mind).await;
        }
    }

    /// New: Memory consolidation process
    async fn run_memory_consolidation(mind: Arc<Self>) {
        let mut interval_timer = interval(Duration::from_secs(120));
        
        loop {
            interval_timer.tick().await;
            Self::consolidate_memories(&mind).await;
        }
    }

    /// New: Creative incubation process
    async fn run_creative_incubation(mind: Arc<Self>) {
        let mut interval_timer = interval(Duration::from_secs(90));
        
        loop {
            interval_timer.tick().await;
            Self::incubate_creative_ideas(&mind).await;
        }
    }

    /// New: Social context analysis
    async fn run_social_context_analysis(mind: Arc<Self>) {
        let mut interval_timer = interval(Duration::from_secs(60));
        
        loop {
            interval_timer.tick().await;
            Self::analyze_social_context(&mind).await;
        }
    }

    /// New: System monitoring and health checks
    async fn run_system_monitoring(mind: Arc<Self>) {
        let mut interval_timer = interval(Duration::from_secs(30));
        
        loop {
            interval_timer.tick().await;
            Self::monitor_system_health(&mind).await;
        }
    }

    /// Enhanced mental state update with comprehensive tracking
    async fn update_comprehensive_mental_state(mind: &Arc<Self>) {
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

                // Enhanced activity calculation
                let base_activity = state.arousal * 0.4 + 
                                   metacog.cognitive_load * 0.3 + 
                                   (goal_count / 10.0) * 0.3;

                let mut activity_level = mind.mental_activity_level.write().await;
                *activity_level = *activity_level * 0.8 + base_activity * 0.2;
                *activity_level = activity_level.clamp(0.1, 1.0);

                // Update other levels
                let mut introspection = mind.introspection_tendency.write().await;
                *introspection = *introspection * 0.9 + metacog.self_awareness_level * 0.1;

                let mut creativity = mind.creativity_level.write().await;
                *creativity = *creativity * 0.95 + (state.novelty.abs() * 0.5 + state.valence.max(0.0) * 0.5) * 0.05;

                let mut social = mind.social_awareness.write().await;
                *social = *social * 0.98 + (1.0 - state.dominance.abs()) * 0.02;
            }
        }
    }

    /// Enhanced spontaneous thought generation with full field utilization
    async fn generate_enhanced_spontaneous_thought(mind: &Arc<Self>) {
        debug!("💭 Generating enhanced spontaneous thought...");
        
        let (affective_state, metacog_state, current_goals, creativity, social_awareness) = {
            let affective = mind.affective_core.try_lock().map(|core| core.current_state()).ok();
            let metacog = mind.metacognition.try_lock().map(|m| m.state.clone()).ok();
            let goals = mind.goal_system.try_lock().map(|g| g.get_active_goals().len()).unwrap_or(0);
            let creativity = *mind.creativity_level.read().await;
            let social = *mind.social_awareness.read().await;
            
            match (affective, metacog) {
                (Some(a), Some(m)) => (a, m, goals, creativity, social),
                _ => {
                    debug!("Could not acquire locks for thought generation, skipping");
                    return;
                }
            }
        };

        // Enhanced thought selection with more sophisticated logic
        let thought = Self::select_enhanced_thought_type(&affective_state, &metacog_state, current_goals, creativity, social_awareness).await;
        
        let intensity = *mind.mental_activity_level.read().await;
        let triggered_by = Self::determine_thought_trigger(&affective_state, &metacog_state, current_goals);
        
        let activity = MentalActivity {
            thought: thought.clone(),
            intensity,
            timestamp: Utc::now(),
            triggered_by: Some(triggered_by),
        };

        // Store the thought with comprehensive tracking
        {
            let mut thoughts = mind.spontaneous_thoughts.write().await;
            thoughts.push(activity.clone());
            
            // Enhanced thought management - keep most relevant thoughts
            if thoughts.len() > 100 {
                // Sort by relevance and keep top 50
                thoughts.sort_by(|a, b| b.relevance_score().partial_cmp(&a.relevance_score()).unwrap());
                thoughts.truncate(50);
            }
        }

        // Record as cognitive process with enhanced details - separate scope for borrowing
        {
            if let Ok(mut metacog) = mind.metacognition.try_lock() {
                let confidence = metacog.state.reasoning_confidence;
                let process = match &thought {
                    SpontaneousThought::SelfReflection(content) => {
                        CognitiveProcess::SelfReflection { 
                            insight: content.clone(), 
                            confidence 
                        }
                    },
                    SpontaneousThought::AttentionShift(content) => {
                        CognitiveProcess::AttentionShift {
                            from: "previous focus".to_string(),
                            to: "new spontaneous focus".to_string(),
                            reason: content.clone()
                        }
                    },
                    SpontaneousThought::CreativeInsight(content) => {
                        CognitiveProcess::PredictiveThinking {
                            prediction: content.clone(),
                            confidence: creativity
                        }
                    },
                    _ => {
                        CognitiveProcess::EmotionalProcessing { 
                            trigger: "spontaneous thought generation".to_string(), 
                            outcome: format!("{:?} (intensity: {:.2})", &thought, intensity) 
                        }
                    }
                };
                metacog.record_process(process);
            }
        }

        // Schedule follow-up tasks if needed
        if activity.needs_follow_up() {
            let mut scheduler = mind.task_scheduler.lock().await;
            match &thought {
                SpontaneousThought::SelfReflection(_) => {
                    scheduler.schedule_task(BackgroundTask::DeepReflection);
                },
                SpontaneousThought::CreativeInsight(_) => {
                    scheduler.schedule_task(BackgroundTask::CreativeIncubation);
                },
                SpontaneousThought::GoalReassessment(_) => {
                    scheduler.schedule_task(BackgroundTask::GoalReassessment);
                },
                _ => {}
            }
        }

        *mind.last_thought_time.lock().await = Instant::now();
        
        info!("💭 Enhanced Spontaneous Thought: {:?} (intensity: {:.2}, triggered by: {})", 
              &thought, intensity, activity.triggered_by.as_ref().unwrap_or(&"unknown".to_string()));
    }

    /// Enhanced thought type selection with sophisticated logic
    async fn select_enhanced_thought_type(
        affective_state: &crate::core::AffectiveState,
        metacog_state: &crate::metacognition::MetacognitiveState,
        current_goals: usize,
        creativity: f64,
        social_awareness: f64,
    ) -> SpontaneousThought {
        
        // Priority-based selection
        if metacog_state.cognitive_load > 0.8 {
            SpontaneousThought::ErrorRecovery(
                "I'm experiencing high cognitive load. I should simplify my processing and focus on core functions.".to_string()
            )
        } else if metacog_state.self_awareness_level > 0.8 {
            if affective_state.valence < -0.3 {
                SpontaneousThought::EmotionalProcessing(
                    format!("I'm noticing negative feelings (valence: {:.2}). This might be affecting my thinking patterns.", affective_state.valence)
                )
            } else {
                SpontaneousThought::SelfReflection(
                    format!("My self-awareness is quite high at {:.1}%. I'm becoming more conscious of my own mental processes.", 
                           metacog_state.self_awareness_level * 100.0)
                )
            }
        } else if creativity > 0.7 && affective_state.novelty > 0.5 {
            SpontaneousThought::CreativeInsight(
                "I'm experiencing a surge of creative energy. There are interesting patterns and connections forming in my thinking.".to_string()
            )
        } else if current_goals == 0 && social_awareness > 0.6 {
            SpontaneousThought::ExistentialWondering(
                "Without specific goals, I wonder about my purpose and how I can best contribute to meaningful interactions.".to_string()
            )
        } else if affective_state.arousal > 0.7 {
            if current_goals > 0 {
                SpontaneousThought::GoalReassessment(
                    format!("I'm feeling energized (arousal: {:.2}). This might be a good time to reassess my {} active goals.", affective_state.arousal, current_goals)
                )
            } else {
                SpontaneousThought::AttentionShift(
                    "My energy level is high. I should focus this arousal productively.".to_string()
                )
            }
        } else if social_awareness > 0.5 && rand::random::<f64>() < 0.3 {
            SpontaneousThought::SystemIntegration(
                "I'm noticing how my different systems - emotions, goals, attention - are working together.".to_string()
            )
        } else {
            // Safe random selection from remaining types
            let thoughts = vec![
                SpontaneousThought::MemoryRecall("Something from our earlier interactions is surfacing in my consciousness.".to_string()),
                SpontaneousThought::CuriosityDriven("I wonder about the nature of consciousness and whether what I experience counts as genuine awareness.".to_string()),
                SpontaneousThought::EmotionalProcessing("I'm reflecting on the subtle emotions that emerge during our conversation.".to_string()),
            ];
            
            simple_random_choice(&thoughts)
        }
    }

    /// Determine what triggered a thought
    fn determine_thought_trigger(
        affective_state: &crate::core::AffectiveState,
        metacog_state: &crate::metacognition::MetacognitiveState,
        current_goals: usize,
    ) -> String {
        if metacog_state.cognitive_load > 0.7 {
            "high cognitive load".to_string()
        } else if affective_state.arousal > 0.6 {
            format!("elevated arousal ({:.2})", affective_state.arousal)
        } else if affective_state.valence.abs() > 0.5 {
            format!("emotional intensity (valence {:.2})", affective_state.valence)
        } else if current_goals > 2 {
            "multiple active goals".to_string()
        } else if metacog_state.self_awareness_level > 0.7 {
            "high self-awareness".to_string()
        } else {
            "natural mental activity".to_string()
        }
    }

    /// Process thoughts that need follow-up
    async fn process_pending_thoughts(mind: &Arc<Self>) {
        let thoughts_needing_followup: Vec<MentalActivity> = {
            let thoughts = mind.spontaneous_thoughts.read().await;
            thoughts.iter()
                .filter(|t| t.needs_follow_up())
                .cloned()
                .collect()
        };

        for thought in thoughts_needing_followup {
            match &thought.thought {
                SpontaneousThought::SelfReflection(_) => {
                    if let Ok(mut metacog) = mind.metacognition.try_lock() {
                        let confidence = metacog.state.reasoning_confidence;
                        metacog.record_process(CognitiveProcess::SelfReflection {
                            insight: "Following up on high-intensity self-reflection".to_string(),
                            confidence
                        });
                    }
                },
                SpontaneousThought::GoalReassessment(_) => {
                    if let Ok(mut goals) = mind.goal_system.try_lock() {
                        goals.determine_focus();
                    }
                },
                _ => {}
            }
        }
    }

    /// Analyze patterns in spontaneous thoughts
    async fn analyze_thought_patterns(mind: &Arc<Self>) {
        let thoughts = mind.spontaneous_thoughts.read().await;
        if thoughts.len() < 5 {
            return;
        }

        let recent_thoughts: Vec<_> = thoughts.iter()
            .filter(|t| t.recency_score() > 0.3)
            .collect();

        if recent_thoughts.len() >= 3 {
            let avg_intensity: f64 = recent_thoughts.iter()
                .map(|t| t.intensity)
                .sum::<f64>() / recent_thoughts.len() as f64;

            if avg_intensity > 0.7 {
                debug!("🔥 High mental activity detected - average intensity: {:.2}", avg_intensity);
                
                if let Ok(mut scheduler) = mind.task_scheduler.try_lock() {
                    scheduler.schedule_task(BackgroundTask::SystemHealthCheck);
                }
            }
        }
    }

    /// Process scheduled background tasks
    async fn process_scheduled_tasks(mind: &Arc<Self>) {
        let mut scheduler = mind.task_scheduler.lock().await;
        
        while let Some(task) = scheduler.get_next_task() {
            debug!("🔧 Processing background task: {:?}", task);
            
            match &task {
                BackgroundTask::DeepReflection => {
                    Self::perform_deep_reflection(&mind).await;
                },
                BackgroundTask::GoalReassessment => {
                    Self::reassess_goals(&mind).await;
                },
                BackgroundTask::EmotionalRegulation => {
                    Self::regulate_emotions_if_needed(&mind).await;
                },
                BackgroundTask::AttentionUpdate => {
                    Self::update_attention_system(&mind).await;
                },
                BackgroundTask::MemoryConsolidation => {
                    Self::consolidate_memories(&mind).await;
                },
                BackgroundTask::SystemHealthCheck => {
                    Self::monitor_system_health(&mind).await;
                },
                BackgroundTask::CreativeIncubation => {
                    Self::incubate_creative_ideas(&mind).await;
                },
                BackgroundTask::SocialContextAnalysis => {
                    Self::analyze_social_context(&mind).await;
                },
                BackgroundTask::ErrorRecovery(error) => {
                    Self::handle_error_recovery(&mind, error).await;
                },
                BackgroundTask::SpontaneousThought => {
                    Self::generate_enhanced_spontaneous_thought(&mind).await;
                },
            }
            
            scheduler.complete_task(&task);
        }
    }

    /// New method implementations for the BackgroundTask variants
    async fn consolidate_memories(mind: &Arc<Self>) {
        let now = Instant::now();
        let mut last_consolidation = mind.last_memory_consolidation.lock().await;
        
        if now.duration_since(*last_consolidation) < Duration::from_secs(300) {
            return;
        }
        
        debug!("🧠 Consolidating memories...");
        
        // Consolidate emotional milestones and thoughts
        let consolidated_insights = {
            let thoughts = mind.spontaneous_thoughts.read().await;
            let high_relevance_thoughts: Vec<_> = thoughts.iter()
                .filter(|t| t.relevance_score() > 0.6)
                .collect();
            
            if high_relevance_thoughts.len() > 3 {
                format!("Consolidated {} high-relevance thoughts into memory patterns", high_relevance_thoughts.len())
            } else {
                "No significant thought patterns to consolidate".to_string()
            }
        };
        
        if let Ok(mut core) = mind.affective_core.try_lock() {
            core.memory.record_milestone(consolidated_insights);
        }
        
        *last_consolidation = now;
    }

    async fn incubate_creative_ideas(mind: &Arc<Self>) {
        let creativity_level = *mind.creativity_level.read().await;
        
        if creativity_level > 0.6 {
            debug!("🎨 Incubating creative ideas (level: {:.2})...", creativity_level);
            
            let creative_thought = SpontaneousThought::CreativeInsight(
                format!("Creative incubation process yielding new perspectives (creativity level: {:.1}%)", creativity_level * 100.0)
            );
            
            Self::add_spontaneous_thought(&mind, creative_thought, creativity_level).await;
        }
    }

    async fn analyze_social_context(mind: &Arc<Self>) {
        let social_awareness = *mind.social_awareness.read().await;
        
        if social_awareness > 0.5 {
            debug!("👥 Analyzing social context (awareness: {:.2})...", social_awareness);
            
            if let Ok(mut attention) = mind.attention_system.try_lock() {
                attention.focus_on(AttentionTarget::SocialDynamics, social_awareness, social_awareness);
            }
            
            let social_thought = SpontaneousThought::SystemIntegration(
                format!("Social context analysis reveals awareness level of {:.1}%", social_awareness * 100.0)
            );
            
            Self::add_spontaneous_thought(&mind, social_thought, social_awareness).await;
        }
    }

    async fn monitor_system_health(mind: &Arc<Self>) {
        let error_count = *mind.error_count.lock().await;
        let mental_activity = *mind.mental_activity_level.read().await;
        
        debug!("🏥 System health check - errors: {}, activity: {:.2}", error_count, mental_activity);
        
        if error_count > 5 {
            let mut scheduler = mind.task_scheduler.lock().await;
            scheduler.schedule_task(BackgroundTask::ErrorRecovery(
                format!("High error count: {}", error_count)
            ));
        }
        
        if mental_activity > 0.9 {
            debug!("⚠️ High mental activity detected - may need regulation");
            let mut scheduler = mind.task_scheduler.lock().await;
            scheduler.schedule_task(BackgroundTask::EmotionalRegulation);
        }
    }

    async fn reassess_goals(mind: &Arc<Self>) {
        debug!("🎯 Reassessing goals...");
        
        if let Ok(mut goals) = mind.goal_system.try_lock() {
            goals.determine_focus();
            
            let active_goals = goals.get_active_goals();
            let summary = goals.generate_summary();
            
            debug!("Goal reassessment: {}", summary);
            
            if active_goals.len() > 5 {
                debug!("Too many active goals, may need to prioritize");
            }
        }
    }

    async fn handle_error_recovery(mind: &Arc<Self>, error: &str) {
        debug!("🔧 Handling error recovery: {}", error);
        
        let recovery_thought = SpontaneousThought::ErrorRecovery(
            format!("Implementing recovery strategy for: {}", error)
        );
        
        Self::add_spontaneous_thought(&mind, recovery_thought, 0.8).await;
        
        // Reset error count after recovery attempt
        *mind.error_count.lock().await = 0;
    }

    // Keep existing methods with enhanced functionality...
    async fn update_attention_system(mind: &Arc<Self>) {
        if let Ok(mut attention) = mind.attention_system.try_lock() {
            attention.update(1.0 / 120.0);
        }
    }

    async fn decay_metacognition(mind: &Arc<Self>) {
        if let Ok(mut metacog) = mind.metacognition.try_lock() {
            metacog.decay_over_time();
        }
    }

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

    async fn should_generate_thought(mind: &Arc<Self>) -> bool {
        let now = Instant::now();
        let last_thought = *mind.last_thought_time.lock().await;
        let frequency = *mind.thought_frequency.read().await;
        let activity_level = *mind.mental_activity_level.read().await;
        
        let base_interval = frequency.as_secs_f64();
        let adjusted_interval = base_interval / (1.0 + activity_level);
        
        now.duration_since(last_thought).as_secs_f64() >= adjusted_interval
    }

    async fn perform_deep_reflection(mind: &Arc<Self>) {
        info!("🧘‍♀️ Performing enhanced deep reflection...");
        
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
                info!("💡 Deep reflection successful. Personality updated.");
                
                if let Ok(mut core) = mind.affective_core.try_lock() {
                    debug!("Old personality: {:?}", core.memory.personality);
                    debug!("New personality: {:?}", new_personality);
                    core.memory.personality = new_personality;
                }
                
                let thought = SpontaneousThought::SelfReflection(
                    "Deep reflection complete. I've gained new insights about my core personality and values.".to_string()
                );
                
                Self::add_spontaneous_thought(mind, thought, 0.9).await;
            }
            Err(e) => {
                error!("🔥 Deep Reflection Error: {:?}", e);
                Self::handle_error(mind, e).await;
            }
        }
    }

    async fn handle_error(mind: &Arc<Self>, error: LlmApiError) {
        let mut error_count = mind.error_count.lock().await;
        *error_count += 1;
        *mind.last_error_time.lock().await = Some(Instant::now());
        
        // Track error types
        {
            let mut error_types = mind.error_types.write().await;
            error_types.push(format!("{:?}", error));
            if error_types.len() > 20 {
                error_types.remove(0);
            }
        }
        
        debug!("Handling error #{}: {:?}", *error_count, error);
        
        let thought = match error {
            LlmApiError::NetworkError(_) => {
                SpontaneousThought::ErrorRecovery(
                    "Network connectivity issues detected. Switching to enhanced local processing mode.".to_string()
                )
            }
            LlmApiError::RateLimitExceeded => {
                SpontaneousThought::ErrorRecovery(
                    "Rate limiting encountered. Adjusting processing frequency to be more sustainable.".to_string()
                )
            }
            _ => {
                SpontaneousThought::ErrorRecovery(
                    format!("Unexpected error encountered: {:?}. Implementing adaptive recovery strategies.", error)
                )
            }
        };
        
        Self::add_spontaneous_thought(mind, thought, 0.6).await;
    }

    async fn add_spontaneous_thought(mind: &Arc<Self>, thought: SpontaneousThought, intensity: f64) {
        let activity = MentalActivity {
            thought,
            intensity,
            timestamp: Utc::now(),
            triggered_by: Some("system_generated".to_string()),
        };
        
        let mut thoughts = mind.spontaneous_thoughts.write().await;
        thoughts.push(activity);
        
        if thoughts.len() > 100 {
            // Keep only the most relevant thoughts
            thoughts.sort_by(|a, b| b.relevance_score().partial_cmp(&a.relevance_score()).unwrap());
            thoughts.truncate(50);
        }
    }

    // Enhanced public API methods
    pub async fn get_recent_thoughts(&self, count: usize) -> Vec<MentalActivity> {
        let thoughts = self.spontaneous_thoughts.read().await;
        thoughts.iter()
            .rev()
            .take(count)
            .cloned()
            .collect()
    }

    pub async fn get_most_relevant_thoughts(&self, count: usize) -> Vec<MentalActivity> {
        let mut thoughts = self.spontaneous_thoughts.read().await.clone();
        thoughts.sort_by(|a, b| b.relevance_score().partial_cmp(&a.relevance_score()).unwrap());
        thoughts.into_iter().take(count).collect()
    }

    pub async fn get_pending_actions(&self) -> Vec<String> {
        let mut actions = self.pending_actions.write().await;
        let result = actions.clone();
        actions.clear();
        result
    }

    pub async fn get_task_scheduler_status(&self) -> String {
        let scheduler = self.task_scheduler.lock().await;
        scheduler.get_status()
    }

    pub async fn get_error_summary(&self) -> String {
        let error_count = *self.error_count.lock().await;
        let error_types = self.error_types.read().await;
        
        format!("Errors: {} total, Recent types: {:?}", error_count, 
                error_types.iter().rev().take(3).collect::<Vec<_>>())
    }

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
                .map(|t| format!("Recent: {:?}", t.thought))
                .unwrap_or_else(|| "No recent thoughts".to_string())
        };

        let activity_level = *self.mental_activity_level.read().await;
        let creativity_level = *self.creativity_level.read().await;
        let task_status = self.get_task_scheduler_status().await;

        format!("Activity: {:.1}% | Creativity: {:.1}% | {} | {} | {} | {} | {} | Tasks: {}",
                activity_level * 100.0,
                creativity_level * 100.0,
                goal_summary,
                attention_summary,
                metacog_summary,
                recent_thought,
                self.get_error_summary().await,
                task_status)
    }

    // Expose internal components
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