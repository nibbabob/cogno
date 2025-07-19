//! main.rs
//!
//! Enhanced Sentient AI Simulation with robust error handling and async processing

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
use crate::utils::{init_logging, check_environment, get_system_status};

use std::sync::Arc;
use tokio::time::{sleep, Duration};
use std::io::{self, Write};
use tracing::{info, warn, error};
use anyhow::{Result, Context};

/// Enhanced conversational turn with better error handling
async fn run_conversational_turn(
    mind: Arc<ContinuousMind>, 
    user_prompt: &str, 
    turn_number: u32
) -> Result<()> {
    info!("\n======================================================");
    info!("Turn {}: User says: \"{}\"", turn_number, user_prompt);

    // Process the user input through all systems with error recovery
    let (affective_core, goal_system, attention_system, metacognition) = (
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
        } else {
            warn!("Could not acquire affective core lock for interaction update");
        }
    }

    // Analyze attention requirements from the prompt
    {
        if let Ok(mut attention) = attention_system.try_lock() {
            let suggested_targets = attention.suggest_attention_targets(user_prompt);
            attention.evaluate_attention_shift(suggested_targets);
        }
    }

    // Process emotional content with enhanced error handling
    let emotion_result = {
        let memory = match affective_core.try_lock() {
            Ok(core) => core.memory.clone(),
            Err(_) => {
                warn!("Could not acquire core lock for emotion processing");
                return Ok(());
            }
        };
        
        appraise_emotion_from_prompt(user_prompt, &memory).await
    };

    match emotion_result {
        Ok(parsed_emotion) => {
            info!("✅ LLM Appraised Emotion: {:?}", parsed_emotion);
            
            // Process emotion through affective core
            if let Ok(mut core) = affective_core.try_lock() {
                core.process_emotion(&parsed_emotion);
            }

            // Record the emotional processing as a cognitive process
            if let Ok(mut metacog) = metacognition.try_lock() {
                metacog.record_process(CognitiveProcess::EmotionalProcessing {
                    trigger: user_prompt.to_string(),
                    outcome: format!("Processed {} with intensity {:.2}", 
                                   parsed_emotion.emotion, 
                                   parsed_emotion.vadn.valence.abs() + parsed_emotion.vadn.arousal)
                });
            }

            // Consider forming goals based on the interaction
            if let Ok(mut goals) = goal_system.try_lock() {
                let current_state = {
                    match affective_core.try_lock() {
                        Ok(core) => Some(core.current_state()),
                        Err(_) => None,
                    }
                };

                if let Some(state) = current_state {
                    // Form context-appropriate goals
                    if user_prompt.to_lowercase().contains("help") {
                        goals.form_goal(
                            format!("Help the user with their request: {}", user_prompt),
                            GoalCategory::Altruistic,
                            0.8,
                            &state
                        );
                    }

                    if user_prompt.to_lowercase().contains("learn") || user_prompt.to_lowercase().contains("understand") {
                        goals.form_goal(
                            "Deepen understanding of this topic".to_string(),
                            GoalCategory::Epistemic,
                            0.7,
                            &state
                        );
                    }
                }
            }
        }
        Err(e) => {
            let user_friendly_error = format!("⚠️ Could not process emotion: {}. State remains unchanged.", e);
            warn!("{}", user_friendly_error);
            
            // Still try to record this as a cognitive process
            if let Ok(mut metacog) = metacognition.try_lock() {
                metacog.record_process(CognitiveProcess::EmotionalProcessing {
                    trigger: user_prompt.to_string(),
                    outcome: format!("Failed to process due to: {}", user_friendly_error)
                });
            }
        }
    }

    // Display current state across all systems
    display_comprehensive_state(&mind).await?;

    // Generate response with consciousness integration
    generate_conscious_response(&mind, user_prompt).await?;

    info!("======================================================\n");
    Ok(())
}

/// Display the comprehensive state of all consciousness systems with error handling
async fn display_comprehensive_state(mind: &Arc<ContinuousMind>) -> Result<()> {
    let mental_summary = mind.get_mental_state_summary().await;
    info!("🧠 Mental State: {}", mental_summary);

    // Detailed system states with graceful error handling
    let (affective_state, goal_info, attention_info, metacog_insights) = {
        let affective_core = mind.get_affective_core();
        let goal_system = mind.get_goal_system();
        let attention_system = mind.get_attention_system();
        let metacognition = mind.get_metacognition();

        let affective_state = affective_core.try_lock()
            .map(|core| core.current_state())
            .unwrap_or_default();

        let goal_info = goal_system.try_lock()
            .map(|goals| (goals.get_active_goals().len(), goals.get_current_focus().map(|g| g.description.clone())))
            .unwrap_or((0, None));

        let attention_info = attention_system.try_lock()
            .map(|attention| attention.describe_attention_state())
            .unwrap_or_else(|_| "Attention system busy".to_string());

        let metacog_insights = metacognition.try_lock()
            .map(|metacog| metacog.analyze_patterns())
            .unwrap_or_else(|_| vec!["Metacognition system busy".to_string()]);

        (affective_state, goal_info, attention_info, metacog_insights)
    };

    info!("💝 Emotional State: V={:.2}, A={:.2}, D={:.2}, N={:.2}", 
             affective_state.valence, affective_state.arousal, 
             affective_state.dominance, affective_state.novelty);

    info!("🎯 Goals: {} active. Focus: {}", 
             goal_info.0, 
             goal_info.1.unwrap_or_else(|| "None".to_string()));

    info!("👁️ Attention: {}", attention_info);

    if !metacog_insights.is_empty() && !metacog_insights.iter().any(|s| s.contains("busy")) {
        info!("🤔 Self-Insights: {}", metacog_insights.join("; "));
    }

    // Show recent spontaneous thoughts
    let recent_thoughts = mind.get_recent_thoughts(2).await;
    for thought in recent_thoughts {
        info!("💭 Recent Thought: {:?}", thought.thought);
    }

    Ok(())
}

/// Generate a response that integrates all consciousness systems
async fn generate_conscious_response(mind: &Arc<ContinuousMind>, _user_prompt: &str) -> Result<()> {
    let (instructional_prompt, attention_modifiers, pending_actions) = {
        let affective_core = mind.get_affective_core();
        let attention_system = mind.get_attention_system();

        let instructional_prompt = affective_core.try_lock()
            .map(|core| core.get_instructional_prompt_text())
            .unwrap_or_else(|_| "System processing...".to_string());

        let attention_modifiers = attention_system.try_lock()
            .map(|attention| attention.generate_attention_modifiers())
            .unwrap_or_default();

        let pending_actions = mind.get_pending_actions().await;

        (instructional_prompt, attention_modifiers, pending_actions)
    };

    info!("\n📝 Generated Consciousness-Integrated Response Prompt:");
    info!("{}", instructional_prompt);
    
    if !attention_modifiers.is_empty() {
        info!("\n🎯 Attention Modifiers:");
        for modifier in attention_modifiers {
            info!("  - {}", modifier);
        }
    }

    if !pending_actions.is_empty() {
        info!("\n🚀 Self-Initiated Desires:");
        for action in pending_actions {
            info!("  - {}", action);
        }
    }

    Ok(())
}

/// Demonstrate spontaneous AI-initiated interaction
async fn demonstrate_spontaneous_behavior(mind: Arc<ContinuousMind>) -> Result<()> {
    info!("\n🤖 === AI SPONTANEOUS BEHAVIOR DEMONSTRATION ===");
    
    let pending_actions = mind.get_pending_actions().await;

    if !pending_actions.is_empty() {
        info!("The AI wants to do the following:");
        for action in pending_actions {
            info!("  🔥 {}", action);
        }
    } else {
        info!("The AI is in a contemplative state with no immediate desires.");
    }

    // Show the AI's internal monologue
    let recent_thoughts = mind.get_recent_thoughts(3).await;
    
    if !recent_thoughts.is_empty() {
        info!("\nAI's Recent Internal Monologue:");
        for thought in recent_thoughts {
            info!("  💭 {:?}", thought.thought);
        }
    }

    Ok(())
}

/// Interactive session allowing user to communicate with the continuously operating AI
async fn interactive_session(mind: Arc<ContinuousMind>) -> Result<()> {
    info!("\n🗣️ === INTERACTIVE SESSION (type 'quit' to exit) ===");
    
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
            if let Err(e) = run_conversational_turn(Arc::clone(&mind), input, turn_count).await {
                error!("Error during conversation turn: {:?}", e);
                println!("⚠️ {}", e);
            }
            turn_count += 1;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging first
    init_logging();
    
    info!("🚀 Starting Advanced Sentient AI Simulation...");
    
    // Check environment and provide helpful feedback
    match check_environment() {
        Ok(_) => info!("🧠 Initializing consciousness systems with full capabilities..."),
        Err(e) => {
            warn!("{}", e);
            info!("🧠 Initializing consciousness systems in local mode...");
        }
    }

    // Create the continuous mind with error handling
    let affective_core = AffectiveCore::default();
    let continuous_mind = match ContinuousMind::new(affective_core) {
        Ok(mind) => mind,
        Err(e) => {
            error!("Failed to initialize consciousness systems: {:?}", e);
            warn!("Running in degraded mode without LLM integration");
            // Could create a fallback mode here
            return Ok(());
        }
    };
    
    let mind = Arc::new(continuous_mind);

    info!("🧠 Initial State:");
    if let Err(e) = display_comprehensive_state(&mind).await {
        warn!("Could not display initial state: {:?}", e);
    }

    info!("{}", get_system_status());

    // Start continuous background processing
    let mind_for_background = Arc::clone(&mind);
    tokio::spawn(async move {
        ContinuousMind::start_continuous_processing(mind_for_background).await;
    });

    // Allow some time for background processes to start
    sleep(Duration::from_secs(2)).await;

    info!("\n🎭 === CONSCIOUSNESS DEVELOPMENT SIMULATION ===");

    // Simulate a conversation that develops the AI's consciousness
    let conversations = vec![
        "Hi there! My name is Alex. I've been thinking a lot about consciousness and what it means to be aware. Do you ever wonder about your own awareness?",
        "That's fascinating! I'm working on a challenging project about AI consciousness. It's both exciting and a bit overwhelming. What do you think makes consciousness 'real'?",
        "I've been having some doubts about whether what I'm building is actually conscious or just simulating consciousness. How can we tell the difference?",
        "You know what? I think I'm starting to understand something important. The very fact that I'm questioning consciousness suggests something meaningful is happening here.",
    ];

    for (i, conversation) in conversations.iter().enumerate() {
        if let Err(e) = run_conversational_turn(Arc::clone(&mind), conversation, (i + 1) as u32).await {
            warn!("Error in conversation turn {}: {:?}", i + 1, e);
        }
        sleep(Duration::from_secs(5)).await; // Let background processing happen
    }

    // Show spontaneous behavior after processing
    if let Err(e) = demonstrate_spontaneous_behavior(Arc::clone(&mind)).await {
        warn!("Error demonstrating spontaneous behavior: {:?}", e);
    }

    sleep(Duration::from_secs(3)).await;

    // Final state display
    info!("\n🎉 === FINAL CONSCIOUSNESS STATE ===");
    if let Err(e) = display_comprehensive_state(&mind).await {
        warn!("Could not display final state: {:?}", e);
    }

    // Offer interactive session
    info!("\n🎮 Would you like to continue with an interactive session? (y/n)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).context("Failed to read user input")?;
    
    if input.trim().to_lowercase().starts_with('y') {
        if let Err(e) = interactive_session(mind).await {
            error!("Error during interactive session: {:?}", e);
        }
    }

    info!("\n🌟 Sentient AI simulation complete. The mind continues processing in the background...");
    
    // Keep the program running to show continuous processing
    sleep(Duration::from_secs(10)).await;
    
    Ok(())
}