//! Inter-agent communication system
//!
//! This module provides message types and communication channels
//! for multi-agent coordination and collaboration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Message types for inter-agent communication
#[derive(Debug, Clone)]
pub enum AgentMessage {
    TaskAssignment {
        task_id: String,
        description: String,
        dependencies: Vec<String>,
    },
    TaskUpdate {
        task_id: String,
        status: TaskStatus,
        output: Option<String>,
    },
    CoordinationRequest {
        from_agent: String,
        request: String,
        context: String,
    },
    SharedArtifact {
        artifact_id: String,
        name: String,
        content: String,
        artifact_type: ArtifactType,
    },
    ObjectiveUpdate {
        objective: String,
        progress: String,
    },
    StatusReport {
        agent_id: String,
        current_work: String,
        next_steps: String,
        help_needed: bool,
    },
    HumanInputRequest {
        from_agent: String,
        request: String,
        context: String,
        urgency: UrgencyLevel,
    },
    InformationShare {
        from_agent: String,
        to_agents: Vec<String>,
        content: String,
        topic: String,
    },
    BlockedNotification {
        agent_id: String,
        blocker: String,
        help_requested: bool,
    },
    ProgressUpdate {
        agent_id: String,
        task_id: String,
        progress_percentage: u8,
        status_message: String,
    },
}

/// Status of a task
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Blocked,
}

/// Type of artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArtifactType {
    Code,
    Document,
    Configuration,
    Test,
    Data,
    Other,
}

/// Urgency level for human input requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UrgencyLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Communication channel for agent messaging
pub struct CommunicationChannel {
    /// Message queue for broadcasting
    pub broadcast_queue: tokio::sync::mpsc::UnboundedSender<AgentMessage>,
    /// Direct message channels
    pub direct_channels: HashMap<String, tokio::sync::mpsc::UnboundedSender<AgentMessage>>,
    /// Message history for context
    pub message_history: Vec<AgentMessage>,
}

impl CommunicationChannel {
    /// Create a new communication channel
    pub fn new() -> (Self, tokio::sync::mpsc::UnboundedReceiver<AgentMessage>) {
        let (broadcast_tx, broadcast_rx) = tokio::sync::mpsc::unbounded_channel();

        (
            Self {
                broadcast_queue: broadcast_tx,
                direct_channels: HashMap::new(),
                message_history: Vec::new(),
            },
            broadcast_rx,
        )
    }

    /// Add a direct channel for an agent
    pub fn add_direct_channel(&mut self, agent_id: String, channel: tokio::sync::mpsc::UnboundedSender<AgentMessage>) {
        self.direct_channels.insert(agent_id, channel);
    }

    /// Broadcast a message to all agents
    pub fn broadcast(&self, message: AgentMessage) -> Result<(), tokio::sync::mpsc::error::SendError<AgentMessage>> {
        self.broadcast_queue.send(message)
    }

    /// Send a direct message to a specific agent
    pub fn send_direct(&self, agent_id: &str, message: AgentMessage) -> Result<(), String> {
        if let Some(channel) = self.direct_channels.get(agent_id) {
            channel.send(message)
                .map_err(|e| format!("Failed to send direct message: {}", e))
        } else {
            Err(format!("No direct channel found for agent: {}", agent_id))
        }
    }

    /// Add message to history
    pub fn add_to_history(&mut self, message: AgentMessage) {
        self.message_history.push(message);
        // Keep only recent messages to manage memory
        if self.message_history.len() > 100 {
            self.message_history.remove(0);
        }
    }

    /// Get recent messages for context
    pub fn get_recent_messages(&self, limit: usize) -> Vec<&AgentMessage> {
        let start = if self.message_history.len() > limit {
            self.message_history.len() - limit
        } else {
            0
        };
        self.message_history[start..].iter().collect()
    }

    /// Get messages by agent
    pub fn get_messages_by_agent(&self, agent_id: &str) -> Vec<&AgentMessage> {
        self.message_history
            .iter()
            .filter(|msg| match msg {
                AgentMessage::TaskUpdate { task_id: _, status: _, output: _ } => false, // TaskUpdate doesn't have agent_id
                AgentMessage::StatusReport { agent_id: msg_agent_id, .. } => msg_agent_id == agent_id,
                AgentMessage::HumanInputRequest { from_agent, .. } => from_agent == agent_id,
                AgentMessage::InformationShare { from_agent, .. } => from_agent == agent_id,
                AgentMessage::BlockedNotification { agent_id: msg_agent_id, .. } => msg_agent_id == agent_id,
                AgentMessage::ProgressUpdate { agent_id: msg_agent_id, .. } => msg_agent_id == agent_id,
                _ => false,
            })
            .collect()
    }

    /// Get messages by topic (from InformationShare)
    pub fn get_messages_by_topic(&self, topic: &str) -> Vec<&AgentMessage> {
        self.message_history
            .iter()
            .filter(|msg| match msg {
                AgentMessage::InformationShare { topic: msg_topic, .. } => msg_topic == topic,
                _ => false,
            })
            .collect()
    }
}

/// Message builder for common communication patterns
pub struct MessageBuilder;

impl MessageBuilder {
    /// Create a status report message
    pub fn status_report(agent_id: String, current_work: String, next_steps: String, help_needed: bool) -> AgentMessage {
        AgentMessage::StatusReport {
            agent_id,
            current_work,
            next_steps,
            help_needed,
        }
    }

    /// Create a human input request
    pub fn human_input_request(from_agent: String, request: String, context: String, urgency: UrgencyLevel) -> AgentMessage {
        AgentMessage::HumanInputRequest {
            from_agent,
            request,
            context,
            urgency,
        }
    }

    /// Create an information share message
    pub fn information_share(from_agent: String, to_agents: Vec<String>, content: String, topic: String) -> AgentMessage {
        AgentMessage::InformationShare {
            from_agent,
            to_agents,
            content,
            topic,
        }
    }

    /// Create a blocked notification
    pub fn blocked_notification(agent_id: String, blocker: String, help_requested: bool) -> AgentMessage {
        AgentMessage::BlockedNotification {
            agent_id,
            blocker,
            help_requested,
        }
    }

    /// Create a progress update
    pub fn progress_update(agent_id: String, task_id: String, progress_percentage: u8, status_message: String) -> AgentMessage {
        AgentMessage::ProgressUpdate {
            agent_id,
            task_id,
            progress_percentage,
            status_message,
        }
    }
}

/// Communication patterns for common scenarios
pub struct CommunicationPatterns;

impl CommunicationPatterns {
    /// Pattern for agent starting work on a task
    pub fn agent_starting_work(agent_id: &str, task_description: &str) -> AgentMessage {
        MessageBuilder::status_report(
            agent_id.to_string(),
            format!("Starting work on: {}", task_description),
            "Will provide updates as work progresses".to_string(),
            false,
        )
    }

    /// Pattern for agent completing a task
    pub fn agent_completing_work(agent_id: &str, task_description: &str, result: &str) -> AgentMessage {
        MessageBuilder::status_report(
            agent_id.to_string(),
            format!("Completed: {}", task_description),
            format!("Result: {}", result),
            false,
        )
    }

    /// Pattern for agent needing human input
    pub fn agent_needs_human_input(agent_id: &str, request: &str, context: &str, urgency: UrgencyLevel) -> AgentMessage {
        MessageBuilder::human_input_request(
            agent_id.to_string(),
            request.to_string(),
            context.to_string(),
            urgency,
        )
    }

    /// Pattern for sharing information with specific agents
    pub fn share_information_with_agents(from_agent: &str, to_agents: Vec<String>, content: &str, topic: &str) -> AgentMessage {
        MessageBuilder::information_share(
            from_agent.to_string(),
            to_agents,
            content.to_string(),
            topic.to_string(),
        )
    }

    /// Pattern for agent being blocked
    pub fn agent_blocked(agent_id: &str, blocker: &str, needs_help: bool) -> AgentMessage {
        MessageBuilder::blocked_notification(
            agent_id.to_string(),
            blocker.to_string(),
            needs_help,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_communication_channel_creation() {
        let (channel, _) = CommunicationChannel::new();
        assert!(channel.direct_channels.is_empty());
        assert!(channel.message_history.is_empty());
    }

    #[test]
    fn test_message_builder() {
        let status_msg = MessageBuilder::status_report(
            "test-agent".to_string(),
            "working on task".to_string(),
            "next steps".to_string(),
            false,
        );

        match status_msg {
            AgentMessage::StatusReport { agent_id, current_work, next_steps, help_needed } => {
                assert_eq!(agent_id, "test-agent");
                assert_eq!(current_work, "working on task");
                assert_eq!(next_steps, "next steps");
                assert!(!help_needed);
            }
            _ => panic!("Expected StatusReport message"),
        }
    }

    #[test]
    fn test_communication_patterns() {
        let start_msg = CommunicationPatterns::agent_starting_work("agent1", "test task");

        match start_msg {
            AgentMessage::StatusReport { agent_id, current_work, .. } => {
                assert_eq!(agent_id, "agent1");
                assert!(current_work.contains("test task"));
            }
            _ => panic!("Expected StatusReport message"),
        }
    }
}