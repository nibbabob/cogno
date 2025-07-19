//! main.rs
//!
//! Main application entry point.
//! This simulates the full middleware loop.

// Declare the modules
mod core;
mod cognitive_appraisal;

use crate::core::AffectiveCore;
use crate::cognitive_appraisal::OccEmotion;

fn main() {
    println!("🚀 Starting Emotional Middleware Simulation...");

    // 1. Initialize the Affective Core using the new `default` implementation
    //    FIX: Changed from `AffectiveCore::new()` to `AffectiveCore::default()`
    let mut affective_core = AffectiveCore::default();
    
    //    FIX: Changed `.current_state` to the `.current_state()` getter method
    println!("🧠 Initial State: {:?}", affective_core.current_state());
    
    // --- SIMULATION OF A CONVERSATIONAL TURN ---
    
    // 2. Simulate User Input and a Gemini API Call
    let user_prompt = "Wow, I can't believe I got the promotion. I worked so hard for this!";
    println!("\nUser says: \"{}\"", user_prompt);
    
    let simulated_llm_response = r#"
        {
            "emotion": "Pride",
            "details": {
                "intensity": 0.9,
                "target": "promotion"
            }
        }
    "#;
    
    // 3. Parse the LLM response into an OccEmotion
    let parsed_emotion: OccEmotion = serde_json::from_str(simulated_llm_response)
        .unwrap_or_default();
    
    println!("✅ LLM Appraised Emotion: {:?}", parsed_emotion);

    // 4. Process the emotion and update the core state
    affective_core.process_emotion(&parsed_emotion);
    //    FIX: Changed to the `.current_state()` getter method
    println!("✨ State after processing emotion: {:?}", affective_core.current_state());

    // 5. Apply emotional regulation (decay toward baseline)
    affective_core.regulate_emotion();
    //    FIX: Changed to the `.current_state()` getter method
    println!("⏳ State after regulation/decay: {:?}", affective_core.current_state());
    //    FIX: Changed `.emotional_history` to the `.history()` getter method
    println!("📜 Emotional History: {:?}", affective_core.history());

    // --- DEMONSTRATE FINAL OUTPUT ---
    
    // 6. Generate the instructional prompt for the LLM's final response.
    //    This is the key output of the core module.
    println!("\n📝 Generated Instructional Microprompt for Gemini:");
    let instructional_prompt = affective_core.get_instructional_prompt_text();
    println!("{}", instructional_prompt);
}