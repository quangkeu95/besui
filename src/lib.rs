//! Onchain automation triggers and workflows
//!
//! Provide an easy way to set alerts and trigger actions on onchain events.

pub mod cli;
mod common;
pub mod config;
pub mod core;
pub mod errors;
pub mod fetcher;
pub mod server;
pub mod token;
