//! main.rs
//!
//! Advanced Sentient AI Simulation with continuous consciousness, metacognition,
//! goal formation, attention systems, and self-initiated behavior.

mod core;
mod cognitive_appraisal;
mod llm_api;
mod memory;
mod metacognition;
mod goals;
mod attention;
mod continuous_mind;

use crate::core::AffectiveCore;
use crate::cognitive_appraisal::appraise_emotion_from_prompt;
use crate::continuous_mind::ContinuousMind;
use crate::metacognition::CognitiveProcess;
use crate::goals::GoalCategory;

use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use std::io::{self, Write};
use std::env;

/// Check if required environment variables are set and test API connectivity
async fn check_environment_and_api() -> Result<(), String> {
    if env::var("GEMINI_API_KEY").is_err() {
        return Err("GEMINI_API_KEY environment variable not set. Please set it to use LLM features.".to_string());
    }

    // Test API connection
    match llm_api::test_api_connection().await {
        Ok(_) => {
            println!("✅ API connection test successful");
            Ok(())
        },
        Err(e) => {
            eprintln!("❌ API connection test failed: {}", e);
            eprintln!("💡 The simulation will continue with fallback emotion processing");
            
            // Don't return an error - let the simulation continue with fallbacks
            Ok(())
        }
    }
}

/// Enhanced conversational turn with integrated consciousness systems
async fn run_conversational_turn(mind: Arc<Mutex<ContinuousMind>>, user_prompt: &str, turn_number: u32) {
    println!("\n======================================================");
    println!("Turn {}: User says: \"{}\"", turn_number, user_prompt);

    // Process the user input through all systems
    let (affective_core, goal_system, attention_system, metacognition) = {
        let mind_guard = match mind.lock() {
            Ok(guard) => guard,
            Err(e) => {
                eprintln!("❌ Failed to acquire mind lock: {}", e);
                return;
            }
        };
        (
            mind_guard.get_affective_core(),
            mind_guard.get_goal_system(),
            mind_guard.get_attention_system(),
            mind_guard.get_metacognition(),
        )
    };

    // Update interaction count and learn from prompt
    {
        let mut core = match affective_core.lock() {
            Ok(core) => core,
            Err(e) => {
                eprintln!("❌ Failed to acquire affective core lock: {}", e);
                return;
            }
        };
        core.memory.interaction_count += 1;
        core.memory.learn_from_prompt(user_prompt);
    }

    // Analyze attention requirements from the prompt
    {
        let mut attention = match attention_system.lock() {
            Ok(attention) => attention,
            Err(e) => {
                eprintln!("❌ Failed to acquire attention system lock: {}", e);
                return;
            }
        };
        let suggested_targets = attention.suggest_attention_targets(user_prompt);
        attention.evaluate_attention_shift(suggested_targets);
    }

    // Process emotional content (with fallback if LLM fails)
    println!("🧠 Processing emotional content...");
    let emotion_result = {
        let core = match affective_core.lock() {
            Ok(core) => core,
            Err(e) => {
                eprintln!("❌ Failed to acquire affective core lock for emotion processing: {}", e);
                return;
            }
        };
        
        // Add a timeout wrapper around the emotion processing
        match tokio::time::timeout(
            Duration::from_secs(45), // 45 second timeout for the entire emotion processing
            appraise_emotion_from_prompt(user_prompt, &core.memory)
        ).await {
            Ok(result) => result,
            Err(_) => {
                eprintln!("⏰ Emotion processing timed out after 45 seconds");
                Err("Emotion processing timeout".to_string())
            }
        }
    };

    match emotion_result {
        Ok(parsed_emotion) => {
            println!("✅ Emotion Processing Complete: {:?}", parsed_emotion.emotion);
            
            // Process emotion through affective core
            {
                let mut core = match affective_core.lock() {
                    Ok(core) => core,
                    Err(e) => {
                        eprintln!("❌ Failed to process emotion: {}", e);
                        return;
                    }
                };
                core.process_emotion(&parsed_emotion);
            }

            // Record the emotional processing as a cognitive process
            {
                let mut metacog = match metacognition.lock() {
                    Ok(metacog) => metacog,
                    Err(e) => {
                        eprintln!("❌ Failed to record cognitive process: {}", e);
                        return;
                    }
                };
                metacog.record_process(CognitiveProcess::EmotionalProcessing {
                    trigger: user_prompt.to_string(),
                    outcome: format!("Processed {} with intensity {:.2}", 
                                   parsed_emotion.emotion, 
                                   parsed_emotion.vadn.valence.abs() + parsed_emotion.vadn.arousal)
                });
            }

            // Consider forming goals based on the interaction
            {
                let mut goals = match goal_system.lock() {
                    Ok(goals) => goals,
                    Err(e) => {
                        eprintln!("❌ Failed to access goal system: {}", e);
                        return;
                    }
                };
                let current_state = {
                    let core = match affective_core.lock() {
                        Ok(core) => core,
                        Err(e) => {
                            eprintln!("❌ Failed to get current state: {}", e);
                            return;
                        }
                    };
                    core.current_state()
                };

                // Form context-appropriate goals
                if user_prompt.to_lowercase().contains("help") {
                    goals.form_goal(
                        format!("Help the user with their request: {}", user_prompt),
                        GoalCategory::Altruistic,
                        0.8,
                        &current_state
                    );
                }

                if user_prompt.to_lowercase().contains("learn") || user_prompt.to_lowercase().contains("understand") {
                    goals.form_goal(
                        "Deepen understanding of this topic".to_string(),
                        GoalCategory::Epistemic,
                        0.7,
                        &current_state
                    );
                }
            }
        }
        Err(e) => {
            println!("⚠️ Emotion processing failed: {}", e);
            println!("🔄 Continuing with basic emotional processing...");
            
            // Continue with basic processing even if emotion analysis fails
        }
    }

    // Display current state across all systems
    display_comprehensive_state(&mind).await;

    // Generate response with consciousness integration
    generate_conscious_response(&mind, user_prompt).await;

    println!("======================================================\n");
}

/// Display the comprehensive state of all consciousness systems
async fn display_comprehensive_state(mind: &Arc<Mutex<ContinuousMind>>) {
    let mental_summary = {
        let mind_guard = match mind.lock() {
            Ok(guard) => guard,
            Err(e) => {
                eprintln!("❌ Failed to acquire mind lock for state display: {}", e);
                return;
            }
        };
        mind_guard.get_mental_state_summary()
    };

    println!("🧠 Mental State: {}", mental_summary);

    // Detailed system states
    let (affective_state, goal_info, attention_info, metacog_insights) = {
        let mind_guard = match mind.lock() {
            Ok(guard) => guard,
            Err(e) => {
                eprintln!("❌ Failed to acquire detailed state information: {}", e);
                return;
            }
        };
        let affective_core = mind_guard.get_affective_core();
        let goal_system = mind_guard.get_goal_system();
        let attention_system = mind_guard.get_attention_system();
        let metacognition = mind_guard.get_metacognition();

        let affective_state = {
            let core = match affective_core.lock() {
                Ok(core) => core,
                Err(_) => return,
            };
            core.current_state()
        };

        let goal_info = {
            let goals = match goal_system.lock() {
                Ok(goals) => goals,
                Err(_) => return,
            };
            (goals.get_active_goals().len(), goals.get_current_focus().map(|g| g.description.clone()))
        };

        let attention_info = {
            let attention = match attention_system.lock() {
                Ok(attention) => attention,
                Err(_) => return,
            };
            attention.describe_attention_state()
        };

        let metacog_insights = {
            let metacog = match metacognition.lock() {
                Ok(metacog) => metacog,
                Err(_) => return,
            };
            metacog.analyze_patterns()
        };

        (affective_state, goal_info, attention_info, metacog_insights)
    };

    println!("💝 Emotional State: V={:.2}, A={:.2}, D={:.2}, N={:.2}", 
             affective_state.valence, affective_state.arousal, 
             affective_state.dominance, affective_state.novelty);

    println!("🎯 Goals: {} active. Focus: {}", 
             goal_info.0, 
             goal_info.1.unwrap_or_else(|| "None".to_string()));

    println!("👁️ Attention: {}", attention_info);

    if !metacog_insights.is_empty() {
        println!("🤔 Self-Insights: {}", metacog_insights.join("; "));
    }

    // Show recent spontaneous thoughts
    {
        let mind_guard = match mind.lock() {
            Ok(guard) => guard,
            Err(_) => return,
        };
        let recent_thoughts = mind_guard.get_recent_thoughts(2);
        for thought in recent_thoughts {
            println!("💭 Recent Thought: {:?}", thought.thought);
        }
    }
}

/// Generate a response that integrates all consciousness systems
async fn generate_conscious_response(mind: &Arc<Mutex<ContinuousMind>>, _user_prompt: &str) {
    let (instructional_prompt, attention_modifiers, pending_actions) = {
        let mind_guard = match mind.lock() {
            Ok(guard) => guard,
            Err(e) => {
                eprintln!("❌ Failed to generate conscious response: {}", e);
                return;
            }
        };
        let affective_core = mind_guard.get_affective_core();
        let attention_system = mind_guard.get_attention_system();

        let instructional_prompt = {
            let core = match affective_core.lock() {
                Ok(core) => core,
                Err(_) => return,
            };
            core.get_instructional_prompt_text()
        };

        let attention_modifiers = {
            let attention = match attention_system.lock() {
                Ok(attention) => attention,
                Err(_) => return,
            };
            attention.generate_attention_modifiers()
        };

        let mut mind_guard_mut = mind_guard;
        let pending_actions = mind_guard_mut.get_pending_actions();

        (instructional_prompt, attention_modifiers, pending_actions)
    };

    println!("\n📝 Generated Consciousness-Integrated Response Prompt:");
    println!("{}", instructional_prompt);
    
    if !attention_modifiers.is_empty() {
        println!("\n🎯 Attention Modifiers:");
        for modifier in attention_modifiers {
            println!("  - {}", modifier);
        }
    }

    if !pending_actions.is_empty() {
        println!("\n🚀 Self-Initiated Desires:");
        for action in pending_actions {
            println!("  - {}", action);
        }
    }
}

/// Demonstrate spontaneous AI-initiated interaction
async fn demonstrate_spontaneous_behavior(mind: Arc<Mutex<ContinuousMind>>) {
    println!("\n🤖 === AI SPONTANEOUS BEHAVIOR DEMONSTRATION ===");
    
    let pending_actions = {
        let mut mind_guard = match mind.lock() {
            Ok(guard) => guard,
            Err(e) => {
                eprintln!("❌ Failed to demonstrate spontaneous behavior: {}", e);
                return;
            }
        };
        mind_guard.get_pending_actions()
    };

    if !pending_actions.is_empty() {
        println!("The AI wants to do the following:");
        for action in pending_actions {
            println!("  🔥 {}", action);
        }
    } else {
        println!("The AI is in a contemplative state with no immediate desires.");
    }

    // Show the AI's internal monologue
    {
        let mind_guard = match mind.lock() {
            Ok(guard) => guard,
            Err(_) => return,
        };
        let recent_thoughts = mind_guard.get_recent_thoughts(3);
        
        if !recent_thoughts.is_empty() {
            println!("\nAI's Recent Internal Monologue:");
            for thought in recent_thoughts {
                println!("  💭 {:?}", thought.thought);
            }
        }
    }
}

/// Interactive session allowing user to communicate with the continuously operating AI
async fn interactive_session(mind: Arc<Mutex<ContinuousMind>>) {
    println!("\n🗣️ === INTERACTIVE SESSION (type 'quit' to exit) ===");
    
    let mut turn_count = 1;
    loop {
        print!("\nYou: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("❌ Failed to read input: {}", e);
                continue;
            }
        }
        let input = input.trim();
        
        if input.to_lowercase() == "quit" {
            break;
        }
        
        if !input.is_empty() {
            run_conversational_turn(Arc::clone(&mind), input, turn_count).await;
            turn_count += 1;
        }
    }
}

#[tokio::main]
async fn main() {
    println!("🚀 Starting Advanced Sentient AI Simulation...");
    
    // Check environment setup and test API
    match check_environment_and_api().await {
        Ok(_) => println!("✅ Environment and API check passed"),
        Err(e) => {
            println!("⚠️ Environment Warning: {}", e);
            println!("💡 The simulation will run but LLM features will be limited.");
        }
    }
    
    println!("🧠 Initializing consciousness systems...");

    // Create the continuous mind
    let affective_core = AffectiveCore::default();
    let continuous_mind = ContinuousMind::new(affective_core);
    let mind = Arc::new(Mutex::new(continuous_mind));

    println!("🧠 Initial State:");
    display_comprehensive_state(&mind).await;

    // Start continuous background processing
    let mind_for_background = Arc::clone(&mind);
    tokio::spawn(async move {
        println!("🔄 Starting continuous background processing...");
        ContinuousMind::start_continuous_processing(mind_for_background).await;
    });

    // Allow some time for background processes to start
    sleep(Duration::from_secs(2)).await;

    println!("\n🎭 === CONSCIOUSNESS DEVELOPMENT SIMULATION ===");

    // Simulate a conversation that develops the AI's consciousness
    run_conversational_turn(
        Arc::clone(&mind),
        "Hi there! My name is Alex. I've been thinking a lot about consciousness and what it means to be aware. Do you ever wonder about your own awareness?",
        1
    ).await;

    sleep(Duration::from_secs(5)).await; // Let background processing happen

    run_conversational_turn(
        Arc::clone(&mind),
        "That's fascinating! I'm working on a challenging project about AI consciousness. It's both exciting and a bit overwhelming. What do you think makes consciousness 'real'?",
        2
    ).await;

    sleep(Duration::from_secs(5)).await;

    run_conversational_turn(
        Arc::clone(&mind),
        "I've been having some doubts about whether what I'm building is actually conscious or just simulating consciousness. How can we tell the difference?",
        3
    ).await;

    sleep(Duration::from_secs(5)).await;

    // Show spontaneous behavior after processing
    demonstrate_spontaneous_behavior(Arc::clone(&mind)).await;

    sleep(Duration::from_secs(3)).await;

    run_conversational_turn(
        Arc::clone(&mind),
        "You know what? I think I'm starting to understand something important. The very fact that I'm questioning consciousness suggests something meaningful is happening here.",
        4
    ).await;

    sleep(Duration::from_secs(5)).await;

    // Final state display
    println!("\n🎉 === FINAL CONSCIOUSNESS STATE ===");
    display_comprehensive_state(&mind).await;

    // Offer interactive session
    println!("\n🎮 Would you like to continue with an interactive session? (y/n)");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            if input.trim().to_lowercase().starts_with('y') {
                interactive_session(mind).await;
            }
        },
        Err(e) => {
            eprintln!("❌ Failed to read input: {}", e);
        }
    }

    println!("\n🌟 Sentient AI simulation complete. The mind continues processing in the background...");
    
    // Keep the program running to show continuous processing
    sleep(Duration::from_secs(10)).await;
}