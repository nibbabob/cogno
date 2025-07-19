//! main.rs
//!
//! Main application entry point. Simulates a conversation and self-reflection.

mod core;
mod cognitive_appraisal;
mod llm_api;
mod memory;

use crate::core::AffectiveCore;
use crate::cognitive_appraisal::appraise_emotion_from_prompt;

async fn run_conversational_turn(core: &mut AffectiveCore, user_prompt: &str) {
    println!("\n------------------------------------------------------");
    core.memory.interaction_count += 1;
    println!("Turn {}: User says: \"{}\"", core.memory.interaction_count, user_prompt);

    core.memory.learn_from_prompt(user_prompt);

    match appraise_emotion_from_prompt(user_prompt, &core.memory).await {
        Ok(parsed_emotion) => {
            println!("✅ LLM Appraised Emotion: {:?}", parsed_emotion);
            core.process_emotion(&parsed_emotion);
        }
        Err(_) => {
            println!("Could not process emotion. State remains unchanged.");
        }
    }
    
    // The core always regulates back towards its (potentially new) baseline personality
    core.regulate_emotion();

    println!("✨ Core Personality: {:?}", core.memory.personality);
    println!("⏳ Final State (after decay): {:?}", core.current_state());
    println!("\n📝 Generated Instructional Microprompt for Gemini:");
    println!("{}", core.get_instructional_prompt_text());
    println!("------------------------------------------------------");
}

#[tokio::main]
async fn main() {
    println!("🚀 Starting Sentient AI Simulation...");

    let mut affective_core = AffectiveCore::default();
    println!("🧠 Initial Personality: {:?}", affective_core.memory.personality);

    // --- SIMULATION OF A CONVERSATION ---

    run_conversational_turn(
        &mut affective_core,
        "Wow, I can't believe I got the promotion. My name is Alex. I worked so hard for this!"
    ).await;

    run_conversational_turn(
        &mut affective_core,
        "This is amazing. I just closed the biggest deal of my career on my first day in the new role!"
    ).await;
    
    run_conversational_turn(
        &mut affective_core,
        "The whole team gave me a round of applause. I feel like I'm on top of the world."
    ).await;

    // --- TRIGGER SELF-REFLECTION ---
    // After a series of positive events, the AI reflects on its experience.
    affective_core.reflect().await;
    
    // --- CONTINUE THE CONVERSATION WITH THE NEW PERSONALITY ---
    
    run_conversational_turn(
        &mut affective_core,
        "Okay, time to focus. I have a new project brief, and it looks pretty challenging."
    ).await;
}