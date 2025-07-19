//! main.rs
//!
//! Main application entry point.
//! This simulates the full middleware loop.

// Declare the modules
mod core;
mod cognitive_appraisal;
mod llm_api;

use crate::core::AffectiveCore;
use crate::cognitive_appraisal::appraise_emotion_from_prompt;

#[tokio::main]
async fn main() {
    println!("🚀 Starting Emotional Middleware Simulation...");

    // 1. Initialize the Affective Core
    let mut affective_core = AffectiveCore::default();
    println!("🧠 Initial State: {:?}", affective_core.current_state());

    // --- SIMULATION OF A CONVERSATIONAL TURN ---

    // 2. Simulate User Input
    let user_prompt = "Wow, I can't believe I got the promotion. I worked so hard for this!";
    println!("\nUser says: \"{}\"", user_prompt);

    // 3. Appraise the emotion from the user's prompt
    let parsed_emotion = appraise_emotion_from_prompt(user_prompt).await;
    println!("✅ LLM Appraised Emotion: {:?}", parsed_emotion);

    // 4. Process the emotion and update the core state
    affective_core.process_emotion(&parsed_emotion);
    println!("✨ State after processing emotion: {:?}", affective_core.current_state());

    // 5. Apply emotional regulation (decay toward baseline)
    affective_core.regulate_emotion();
    println!("⏳ State after regulation/decay: {:?}", affective_core.current_state());
    println!("📜 Emotional History: {:?}", affective_core.history());

    // --- DEMONSTRATE FINAL OUTPUT ---

    // 6. Generate the instructional prompt for the LLM's final response.
    println!("\n📝 Generated Instructional Microprompt for Gemini:");
    let instructional_prompt = affective_core.get_instructional_prompt_text();
    println!("{}", instructional_prompt);
}