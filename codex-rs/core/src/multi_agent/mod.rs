//! Casual Multi-Agent Collaboration System for Codex
//!
//! This module provides a simplified multi-agent system where:
//! - User provides a single objective
//! - LLM dynamically creates agent team and task plan
//! - AI agents collaborate autonomously
//! - Human can casually engage without formal joining
//! - Zero breaking changes to standalone Codex

pub mod agents;
pub mod casual;
pub mod communication;

// Re-export main types for easy access
pub use agents::AgentManager;
pub use casual::{CasualMultiAgentOrchestrator, CasualTaskSession, CasualAction, CasualAgent};
pub use communication::AgentMessage;