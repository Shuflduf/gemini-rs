#![warn(unreachable_pub, unused_qualifications)]

//! *A Rust client library for Google's Gemini AI models.*
//!
//! # Overview
//!
//! This library provides a fully-featured client for interacting with Google's Gemini AI models,
//! supporting all major API features including:
//!
//! - Text generation and chat conversations
//! - JSON-structured outputs
//! - Function calling
//! - Safety settings and content filtering
//! - System instructions
//! - Model configuration (temperature, tokens, etc.)
//!
//! # Authentication
//!
//! The client requires a Gemini API key which can be provided in two ways:
//! - Environment variable: `GEMINI_API_KEY`
//! - Programmatically: `Client::new(api_key)`
//!
//! # Basic Usage
//!
//! ```rust,no_run
//! #[tokio::main]
//! async fn main() -> gemini_rs::Result<()> {
//!     // Simple chat interaction
//!     let response = gemini_rs::chat("gemini-2.0-flash")
//!         .send_message("What is Rust's ownership model?")
//!         .await?;
//!     println!("{}", response);
//!
//!     Ok(())
//! }
//! ```
//!
//! # Advanced Features
//!
//! The library supports advanced Gemini features through the `Client` and `Chat` types:
//!
//! - Model management (`client().models()`)
//! - Custom generation settings (`chat.config_mut()`)
//! - Safety settings (`chat.safety_settings()`)
//! - System instructions (`chat.system_instruction()`)
//! - Conversation history management (`chat.history_mut()`)

mod chat;
mod client;
mod error;
pub mod types;

pub type Result<T> = std::result::Result<T, Error>;

pub use chat::Chat;
pub use client::Client;
pub use error::Error;

/// Creates a new Gemini client instance using the default configuration.
pub fn client() -> Client {
    Client::instance()
}

/// Creates a new chat session with the specified Gemini model.
pub fn chat(model: &str) -> Chat<chat::Text> {
    client().chat(model)
}
