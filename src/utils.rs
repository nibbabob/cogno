//! utils.rs
//!
//! Utility functions for logging, error handling, and system setup.

use tracing::{info, Level};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use std::env;

/// Initialize logging system with appropriate levels
pub fn init_logging() {
    let filter = EnvFilter::from_default_env()
        .add_directive("cogno=info".parse().unwrap())
        .add_directive("reqwest=warn".parse().unwrap())
        .add_directive("hyper=warn".parse().unwrap());

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(false)
        .with_line_number(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    info!("🔧 Logging system initialized");
}

/// Check if required environment variables are set
pub fn check_environment() -> Result<(), String> {
    if env::var("GEMINI_API_KEY").is_err() {
        return Err("GEMINI_API_KEY environment variable not set. Please set it to use LLM features.".to_string());
    }
    
    info!("✅ Environment variables validated");
    Ok(())
}

/// Format error messages for user display
pub fn format_error_for_user<E: std::fmt::Display>(error: &E) -> String {
    let error_str = error.to_string();
    
    if error_str.contains("API key") {
        "🔑 API configuration issue. Please check your environment variables.".to_string()
    } else if error_str.contains("Network") || error_str.contains("timeout") {
        "🌐 Network connectivity issue. The system will continue with local processing.".to_string()
    } else if error_str.contains("rate limit") {
        "⏳ Service is busy. The system will slow down requests automatically.".to_string()
    } else {
        format!("⚠️ {}", error_str)
    }
}

/// Get a user-friendly system status
pub fn get_system_status() -> String {
    let api_status = if env::var("GEMINI_API_KEY").is_ok() {
        "🟢 API Ready"
    } else {
        "🟡 API Not Configured (local mode)"
    };
    
    format!("System Status: {} | Memory: 🟢 Active | Processing: 🟢 Running", api_status)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_formatting() {
        let error = std::io::Error::new(std::io::ErrorKind::TimedOut, "Network timeout");
        let formatted = format_error_for_user(&error);
        assert!(formatted.contains("Network connectivity"));
    }

    #[test]
    fn test_system_status() {
        let status = get_system_status();
        assert!(status.contains("System Status"));
    }
}