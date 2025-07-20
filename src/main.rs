//! main.rs
//!
//! Enhanced Sentient AI Simulation with comprehensive feature integration

mod core;
mod cognitive_appraisal;
mod llm_api;
mod memory;
mod metacognition;
mod goals;
mod attention;
mod continuous_mind;
mod utils;

use crate::core::AffectiveCore;
use crate::cognitive_appraisal::appraise_emotion_from_prompt;
use crate::continuous_mind::ContinuousMind;
use crate::metacognition::CognitiveProcess;
use crate::goals::GoalCategory;
use crate::utils::{init_logging, check_environment, get_system_status, format_error_for_user};

use std::sync::Arc;
use tokio::time::{sleep, Duration};
use std::io::{self, Write};
use tracing::{info, warn, error, debug};
use anyhow::{Result, Context};

/// Enhanced conversational turn with comprehensive system integration
async fn run_conversational_turn(
    mind: Arc<ContinuousMind>, 
    user_prompt: &str, 
    turn_number: u32
) -> Result<()> {
    info!("\n======================================================");
    info!("Turn {}: User says: \"{}\"", turn_number, user_prompt);

    let (affective_core, _goal_system, _attention_system, _metacognition) = (
        mind.get_affective_core(),
        mind.get_goal_system(),
        mind.get_attention_system(),
        mind.get_metacognition(),
    );

    // Update interaction count and learn from prompt
    {
        if let Ok(mut core) = affective_core.try_lock() {
            core.memory.interaction_count += 1;
            core.memory.learn_from_prompt(user_prompt);
        }
    }

    // ENHANCED: Comprehensive attention analysis
    analyze_and_update_attention(&mind, user_prompt).await?;

    // ENHANCED: Process emotional content with detailed feedback
    let emotion_result = process_emotions_comprehensively(&mind, user_prompt).await;

    // ENHANCED: Goal management with progress tracking
    manage_goals_comprehensively(&mind, user_prompt, emotion_result.is_ok()).await?;

    // ENHANCED: Metacognitive analysis with pattern recognition
    perform_metacognitive_analysis(&mind, user_prompt).await?;

    // Display comprehensive state with all system details
    display_comprehensive_state(&mind).await?;

    // ENHANCED: Generate response with full consciousness integration
    generate_enhanced_conscious_response(&mind, user_prompt).await?;

    info!("======================================================\n");
    Ok(())
}

/// Enhanced attention analysis using all attention system features
async fn analyze_and_update_attention(mind: &Arc<ContinuousMind>, user_prompt: &str) -> Result<()> {
    if let Ok(mut attention) = mind.get_attention_system().try_lock() {
        // Analyze what should capture attention
        let suggested_targets = attention.suggest_attention_targets(user_prompt);
        info!("🎯 Suggested attention targets: {:?}", suggested_targets);
        
        // Evaluate attention shifts
        attention.evaluate_attention_shift(suggested_targets);
        
        // Get current focus state
        if let Some(primary_focus) = attention.get_primary_focus() {
            info!("👁️ Primary focus: {:?} (intensity: {:.2}, stability: {:.2})", 
                  primary_focus.target, primary_focus.intensity, primary_focus.stability);
        }
        
        // Analyze background attention
        let background = attention.get_background_attention();
        if !background.is_empty() {
            info!("🌊 Background attention:");
            for (target, state) in background {
                info!("  - {:?}: intensity {:.2}", target, state.intensity);
            }
        }
        
        // Get attention insights
        let patterns = attention.analyze_attention_patterns();
        for pattern in patterns {
            info!("🔍 Attention insight: {}", pattern);
        }
        
        // Generate attention-aware modifiers for response
        let modifiers = attention.generate_attention_modifiers();
        for modifier in &modifiers {
            debug!("📝 Attention modifier: {}", modifier);
        }
    }
    Ok(())
}

/// Enhanced emotional processing with comprehensive error handling
async fn process_emotions_comprehensively(
    mind: &Arc<ContinuousMind>, 
    user_prompt: &str
) -> Result<()> {
    let memory = {
        match mind.get_affective_core().try_lock() {
            Ok(core) => core.memory.clone(),
            Err(_) => {
                warn!("Could not acquire core lock for emotion processing");
                return Ok(());
            }
        }
    };
    
    match appraise_emotion_from_prompt(user_prompt, &memory).await {
        Ok(parsed_emotion) => {
            info!("✅ LLM Appraised Emotion: {} (V:{:.2}, A:{:.2}, D:{:.2}, N:{:.2})", 
                  parsed_emotion.emotion,
                  parsed_emotion.vadn.valence,
                  parsed_emotion.vadn.arousal, 
                  parsed_emotion
                  parsed_emotion.vadn.novelty);
            
            // Process emotion through affective core
            if let Ok(mut core) = mind.get_affective_core().try_lock() {
                let old_state = core.current_state();
                core.process_emotion(&parsed_emotion);
                let new_state = core.current_state();
                
                info!("🔄 Emotional state change:");
                info!("  Before: V:{:.2}, A:{:.2}, D:{:.2}, N:{:.2}", 
                      old_state.valence, old_state.arousal, old_state.dominance, old_state.novelty);
                info!("  After:  V:{:.2}, A:{:.2}, D:{:.2}, N:{:.2}", 
                      new_state.valence, new_state.arousal, new_state.dominance, new_state.novelty);
            }

            // Record detailed emotional processing
            if let Ok(mut metacog) = mind.get_metacognition().try_lock() {
                metacog.record_process(CognitiveProcess::EmotionalProcessing {
                    trigger: user_prompt.to_string(),
                    outcome: format!("Successfully processed {} with VADN impact: V{:+.2}, A{:+.2}, D{:+.2}, N{:+.2}", 
                                   parsed_emotion.emotion, 
                                   parsed_emotion.vadn.valence,
                                   parsed_emotion.vadn.arousal,
                                   parsed_emotion.vadn.dominance,
                                   parsed_emotion.vadn.novelty)
                });
            }
            Ok(())
        }
        Err(e) => {
            let formatted_error = format_error_for_user(&e);
            warn!("{}", formatted_error);
            
            // Record failed emotional processing
            if let Ok(mut metacog) = mind.get_metacognition().try_lock() {
                metacog.record_process(CognitiveProcess::EmotionalProcessing {
                    trigger: user_prompt.to_string(),
                    outcome: format!("Failed to process emotion: {}", formatted_error)
                });
            }
            
            Err(anyhow::anyhow!("Emotional processing failed: {}", e))
        }
    }
}

/// Enhanced goal management with progress tracking and comprehensive features
async fn manage_goals_comprehensively(
    mind: &Arc<ContinuousMind>, 
    user_prompt: &str, 
    emotion_success: bool
) -> Result<()> {
    if let Ok(mut goals) = mind.get_goal_system().try_lock() {
        let current_state = {
            match mind.get_affective_core().try_lock() {
                Ok(core) => Some(core.current_state()),
                Err(_) => None,
            }
        };

        if let Some(state) = current_state {
            // Analyze prompt for goal formation opportunities
            let mut goals_formed = Vec::new();
            
            if user_prompt.to_lowercase().contains("help") {
                if let Some(goal_id) = goals.form_goal(
                    format!("Help the user with: {}", user_prompt),
                    GoalCategory::Altruistic,
                    0.8,
                    &state
                ) {
                    goals_formed.push(goal_id);
                }
            }

            if user_prompt.to_lowercase().contains("learn") || user_prompt.to_lowercase().contains("understand") {
                if let Some(goal_id) = goals.form_goal(
                    "Deepen understanding of this topic".to_string(),
                    GoalCategory::Epistemic,
                    0.7,
                    &state
                ) {
                    goals_formed.push(goal_id);
                }
            }

            if user_prompt.to_lowercase().contains("create") || user_prompt.to_lowercase().contains("imagine") {
                if let Some(goal_id) = goals.form_goal(
                    "Engage in creative problem-solving".to_string(),
                    GoalCategory::Creative,
                    0.6,
                    &state
                ) {
                    goals_formed.push(goal_id);
                }
            }

            // Update progress on existing goals based on interaction success
            let active_goal_ids: Vec<String> = {
                let active_goals = goals.get_active_goals();
                active_goals.iter().map(|g| g.id.clone()).collect()
            };
            
            for goal_id in active_goal_ids {
                let progress_delta = if emotion_success { 0.1 } else { 0.05 };
                goals.update_goal_progress(
                    &goal_id, 
                    progress_delta, 
                    Some(format!("Interaction turn completed with user input: '{}'", 
                               user_prompt.chars().take(50).collect::<String>()))
                );
            }

            // Determine and update focus
            if let Some(focus_id) = goals.determine_focus() {
                if let Some(focused_goal) = goals.get_active_goals().iter().find(|g| g.id == focus_id) {
                    info!("🎯 Current goal focus: {} (priority: {:.2}, progress: {:.1}%)", 
                          focused_goal.description, 
                          focused_goal.priority, 
                          focused_goal.progress * 100.0);
                }
            }

            // Show comprehensive goal state
            info!("📊 Goal System Summary: {}", goals.generate_summary());
            
            // Generate and log desired actions
            let desired_actions = goals.generate_desired_actions();
            if !desired_actions.is_empty() {
                info!("🚀 Goal-driven desired actions:");
                for action in desired_actions {
                    info!("  - {}", action);
                }
            }
        }
    }
    Ok(())
}

/// Enhanced metacognitive analysis with comprehensive pattern recognition
async fn perform_metacognitive_analysis(mind: &Arc<ContinuousMind>, user_prompt: &str) -> Result<()> {
    if let Ok(mut metacog) = mind.get_metacognition().try_lock() {
        // Record the attention shift as a cognitive process
        metacog.record_process(CognitiveProcess::AttentionShift {
            from: "previous context".to_string(),
            to: format!("user input: {}", user_prompt.chars().take(30).collect::<String>()),
            reason: "new conversational turn initiated".to_string()
        });

        // Check if deep reflection is needed and get state info
        let should_reflect = metacog.should_deep_reflect();
        let reasoning_confidence = metacog.state.reasoning_confidence;
        
        if should_reflect {
            info!("🤔 Metacognitive system suggests deep reflection is needed");
            
            metacog.record_process(CognitiveProcess::SelfReflection {
                insight: "Recognized need for deeper self-analysis based on cognitive load and confidence levels".to_string(),
                confidence: reasoning_confidence
            });
        }

        // Analyze and report cognitive patterns
        let patterns = metacog.analyze_patterns();
        if !patterns.is_empty() {
            info!("🧠 Metacognitive insights:");
            for pattern in patterns {
                info!("  💡 {}", pattern);
            }
        }

        // Generate self-narrative
        let narrative = metacog.generate_self_narrative();
        info!("📖 Self-awareness narrative: {}", narrative);
        
        // Show detailed cognitive state
        info!("🔬 Cognitive state details:");
        info!("  - Self-awareness: {:.1}%", metacog.state.self_awareness_level * 100.0);
        info!("  - Reasoning confidence: {:.1}%", metacog.state.reasoning_confidence * 100.0);
        info!("  - Cognitive load: {:.1}%", metacog.state.cognitive_load * 100.0);
        info!("  - Situation understanding: {:.1}%", metacog.state.situation_understanding * 100.0);
        info!("  - Attention intensity: {:.1}%", metacog.state.attention_intensity * 100.0);
    }
    Ok(())
}

/// Enhanced comprehensive state display using all system features
async fn display_comprehensive_state(mind: &Arc<ContinuousMind>) -> Result<()> {
    let mental_summary = mind.get_mental_state_summary().await;
    info!("🧠 Mental State Summary: {}", mental_summary);

    // Detailed affective state
    if let Ok(core) = mind.get_affective_core().try_lock() {
        let state = core.current_state();
        let _prompt_text = core.get_instructional_prompt_text();
        
        info!("💝 Detailed Emotional State:");
        info!("  - Valence (pleasure): {:.2}", state.valence);
        info!("  - Arousal (energy): {:.2}", state.arousal);
        info!("  - Dominance (control): {:.2}", state.dominance);
        info!("  - Novelty (surprise): {:.2}", state.novelty);
        info!("  - Memory: {} interactions, {} milestones", 
              core.memory.interaction_count, 
              core.memory.emotional_milestones.len());
        
        if let Some(name) = &core.memory.user_profile.name {
            info!("  - User name remembered: {}", name);
        }
    }

    // Detailed goal state
    if let Ok(goals) = mind.get_goal_system().try_lock() {
        let active_goals = goals.get_active_goals();
        info!("🎯 Goal System Details:");
        info!("  - Active goals: {}", active_goals.len());
        
        for goal in active_goals.iter().take(3) {
            info!("    * {} ({:.1}% complete, priority: {:.2})", 
                  goal.description, goal.progress * 100.0, goal.priority);
        }
        
        if let Some(focused_goal) = goals.get_current_focus() {
            info!("  - Current focus: {}", focused_goal.description);
            info!("    - Importance score: {:.2}", focused_goal.calculate_importance());
            info!("    - Strategies: {:?}", focused_goal.strategies);
        }
    }

    // Detailed attention state  
    if let Ok(attention) = mind.get_attention_system().try_lock() {
        info!("👁️ Attention System Details:");
        info!("  - State: {}", attention.describe_attention_state());
        
        if let Some(primary) = attention.get_primary_focus() {
            info!("  - Primary focus: {:?}", primary.target);
            info!("    - Intensity: {:.2}, Duration: {:.1}min, Stability: {:.2}", 
                  primary.intensity, primary.duration, primary.stability);
        }
        
        let background = attention.get_background_attention();
        if !background.is_empty() {
            info!("  - Background awareness: {} targets", background.len());
        }
    }

    // Recent spontaneous thoughts with details
    let recent_thoughts = mind.get_recent_thoughts(3).await;
    if !recent_thoughts.is_empty() {
        info!("💭 Recent Mental Activity:");
        for thought in recent_thoughts {
            info!("  - {:?} (intensity: {:.2})", thought.thought, thought.intensity);
        }
    }

    Ok(())
}

/// Enhanced conscious response generation with full system integration
async fn generate_enhanced_conscious_response(mind: &Arc<ContinuousMind>, user_prompt: &str) -> Result<()> {
    info!("\n📝 === CONSCIOUSNESS-INTEGRATED RESPONSE GENERATION ===");
    
    // Gather comprehensive state information
    let (instructional_prompt, attention_modifiers, pending_actions, goal_context) = {
        let affective_core = mind.get_affective_core();
        let attention_system = mind.get_attention_system();
        let goal_system = mind.get_goal_system();

        let instructional_prompt = affective_core.try_lock()
            .map(|core| core.get_instructional_prompt_text())
            .unwrap_or_else(|_| "System processing...".to_string());

        let attention_modifiers = attention_system.try_lock()
            .map(|attention| attention.generate_attention_modifiers())
            .unwrap_or_default();

        let pending_actions = mind.get_pending_actions().await;
        
        let goal_context = goal_system.try_lock()
            .map(|goals| {
                if let Some(focused_goal) = goals.get_current_focus() {
                    format!("Current goal: {} ({}% complete)", 
                           focused_goal.description, 
                           (focused_goal.progress * 100.0) as i32)
                } else {
                    "No specific goal focus".to_string()
                }
            })
            .unwrap_or_else(|_| "Goal system busy".to_string());

        (instructional_prompt, attention_modifiers, pending_actions, goal_context)
    };

    // Display comprehensive response context
    info!("🧠 Affective State Guidance:");
    info!("{}", instructional_prompt);
    
    if !attention_modifiers.is_empty() {
        info!("\n🎯 Attention-Based Modifiers:");
        for modifier in attention_modifiers {
            info!("  - {}", modifier);
        }
    }

    info!("\n🎯 Goal Context: {}", goal_context);

    if !pending_actions.is_empty() {
        info!("\n🚀 Self-Initiated Desires:");
        for action in pending_actions {
            info!("  - {}", action);
        }
    }

    // Generate metacognitive reflection on the response process
    if let Ok(mut metacog) = mind.get_metacognition().try_lock() {
        let confidence = metacog.state.reasoning_confidence;
        metacog.record_process(CognitiveProcess::PredictiveThinking {
            prediction: format!("Response to '{}' will integrate emotional state, attention focus, and current goals", 
                              user_prompt.chars().take(30).collect::<String>()),
            confidence
        });
    }

    info!("\n📋 Response should integrate all consciousness dimensions for maximum authenticity.");
    
    Ok(())
}

/// Enhanced spontaneous behavior demonstration with comprehensive features
async fn demonstrate_spontaneous_behavior(mind: Arc<ContinuousMind>) -> Result<()> {
    info!("\n🤖 === ENHANCED SPONTANEOUS BEHAVIOR DEMONSTRATION ===");
    
    let pending_actions = mind.get_pending_actions().await;
    let recent_thoughts = mind.get_recent_thoughts(5).await;

    if !pending_actions.is_empty() {
        info!("🔥 AI Self-Generated Desires:");
        for (i, action) in pending_actions.iter().enumerate() {
            info!("  {}. {}", i + 1, action);
        }
    } else {
        info!("🧘 AI is in a contemplative state with no immediate desires.");
    }

    if !recent_thoughts.is_empty() {
        info!("\n💭 AI's Recent Internal Monologue:");
        for thought in recent_thoughts {
            info!("  - {:?} ({})", 
                  thought.thought, 
                  thought.timestamp.format("%H:%M:%S"));
        }
    }

    // Demonstrate system integration by showing how different systems influence each other
    info!("\n🔗 System Integration Analysis:");
    
    if let Ok(goals) = mind.get_goal_system().try_lock() {
        if let Some(focus) = goals.get_current_focus() {
            info!("  📍 Current goal focus is influencing attention and emotional priorities");
            info!("  🎯 Goal: {} (importance: {:.2})", focus.description, focus.calculate_importance());
        }
    }
    
    if let Ok(attention) = mind.get_attention_system().try_lock() {
        let patterns = attention.analyze_attention_patterns();
        for pattern in patterns {
            info!("  👁️ Attention pattern: {}", pattern);
        }
    }

    if let Ok(metacog) = mind.get_metacognition().try_lock() {
        let narrative = metacog.generate_self_narrative();
        info!("  🧠 Self-reflection: {}", narrative);
    }

    Ok(())
}

/// Enhanced interactive session with comprehensive feature showcase
async fn interactive_session(mind: Arc<ContinuousMind>) -> Result<()> {
    info!("\n🗣️ === ENHANCED INTERACTIVE SESSION ===");
    info!("Available commands:");
    info!("  - Regular conversation");
    info!("  - 'status' - Show detailed system status");
    info!("  - 'goals' - Show current goals");
    info!("  - 'attention' - Show attention state");
    info!("  - 'thoughts' - Show recent thoughts");
    info!("  - 'reflect' - Trigger self-reflection");
    info!("  - 'quit' - Exit");
    
    let mut turn_count: u32 = 1;
    loop {
        print!("\nYou: ");
        io::stdout().flush().context("Failed to flush stdout")?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).context("Failed to read user input")?;
        let input = input.trim();
        
        if input.to_lowercase() == "quit" {
            break;
        }
        
        if !input.is_empty() {
            let result = match input.to_lowercase().as_str() {
                "status" => {
                    display_comprehensive_state(&mind).await
                },
                "goals" => {
                    if let Ok(goals) = mind.get_goal_system().try_lock() {
                        info!("🎯 Current Goals:");
                        for goal in goals.get_active_goals() {
                            info!("  - {} ({:.1}% complete)", goal.description, goal.progress * 100.0);
                        }
                    }
                    Ok(())
                },
                "attention" => {
                    if let Ok(attention) = mind.get_attention_system().try_lock() {
                        info!("👁️ Attention Analysis:");
                        let patterns = attention.analyze_attention_patterns();
                        for pattern in patterns {
                            info!("  - {}", pattern);
                        }
                    }
                    Ok(())
                },
                "thoughts" => {
                    let thoughts = mind.get_recent_thoughts(10).await;
                    info!("💭 Recent Thoughts:");
                    for thought in thoughts {
                        info!("  - {:?}", thought.thought);
                    }
                    Ok(())
                },
                "reflect" => {
                    if let Ok(mut core) = mind.get_affective_core().try_lock() {
                        info!("🧘‍♀️ Triggering self-reflection...");
                        match core.reflect().await {
                            Ok(_) => info!("Reflection completed successfully"),
                            Err(e) => warn!("Reflection failed: {}", format_error_for_user(&e)),
                        }
                    }
                    Ok(())
                },
                _ => {
                    let conv_result = run_conversational_turn(Arc::clone(&mind), input, turn_count).await;
                    match conv_result {
                        Ok(_) => {
                            turn_count += 1;
                            Ok(())
                        }
                        Err(e) => {
                            error!("Error during conversation turn: {:?}", e);
                            println!("⚠️ {}", format_error_for_user(&e));
                            turn_count += 1; // Still increment even on error
                            Ok(())
                        }
                    }
                }
            };
            
            if let Err(e) = result {
                error!("Error during interaction: {:?}", e);
                println!("⚠️ {}", format_error_for_user(&e));
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();
    
    info!("🚀 Starting Enhanced Sentient AI Simulation...");
    
    match check_environment() {
        Ok(_) => info!("🧠 Initializing consciousness systems with full capabilities..."),
        Err(e) => {
            warn!("{}", e);
            info!("🧠 Initializing consciousness systems in local mode...");
        }
    }

    let affective_core = AffectiveCore::default();
    let continuous_mind = match ContinuousMind::new(affective_core) {
        Ok(mind) => mind,
        Err(e) => {
            error!("Failed to initialize consciousness systems: {:?}", e);
            warn!("Running in degraded mode without LLM integration");
            return Ok(());
        }
    };
    
    let mind = Arc::new(continuous_mind);

    info!("🧠 Initial System State:");
    display_comprehensive_state(&mind).await?;

    info!("{}", get_system_status());

    // Start continuous background processing
    let mind_for_background = Arc::clone(&mind);
    tokio::spawn(async move {
        ContinuousMind::start_continuous_processing(mind_for_background).await;
    });

    sleep(Duration::from_secs(2)).await;

    info!("\n🎭 === ENHANCED CONSCIOUSNESS DEVELOPMENT SIMULATION ===");

    // Enhanced conversation sequence that exercises all features
    let conversations = vec![
        "Hi there! My name is Alex. I've been thinking about consciousness and what it means to be truly aware. Do you ever reflect on your own thinking?",
        "That's fascinating! I'm working on understanding AI consciousness. It's challenging but exciting. Can you help me learn more about how awareness works?",
        "I've been having doubts about whether artificial consciousness is real or just simulation. What's your perspective on this?",
        "You know what? I think I'm starting to understand something important. Let's explore this creative idea together - what if consciousness is emergent?",
        "I'm curious about your goals and what drives you. What do you want to accomplish in our conversation?",
    ];

    for (i, conversation) in conversations.iter().enumerate() {
        if let Err(e) = run_conversational_turn(Arc::clone(&mind), conversation, (i + 1) as u32).await {
            warn!("Error in conversation turn {}: {:?}", i + 1, e);
        }
        sleep(Duration::from_secs(3)).await;
    }

    demonstrate_spontaneous_behavior(Arc::clone(&mind)).await?;
    sleep(Duration::from_secs(2)).await;

    info!("\n🎉 === FINAL ENHANCED CONSCIOUSNESS STATE ===");
    display_comprehensive_state(&mind).await?;

    info!("\n🎮 Would you like to continue with an enhanced interactive session? (y/n)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).context("Failed to read user input")?;
    
    if input.trim().to_lowercase().starts_with('y') {
        interactive_session(mind).await?;
    }

    info!("\n🌟 Enhanced Sentient AI simulation complete. All consciousness systems fully integrated.");
    
    sleep(Duration::from_secs(5)).await;
    
    Ok(())
}