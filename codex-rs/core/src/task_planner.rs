//! Task planning system for multi-agent coordination
//!
//! This module provides intelligent task decomposition and assignment
//! for multi-agent workflows.

use anyhow::Result;
use std::collections::HashMap;

use crate::multi_agent::agents::AgentProfile;

/// Individual subtask in a task plan
#[derive(Debug, Clone)]
pub struct Subtask {
    pub id: String,
    pub description: String,
    pub required_capabilities: Vec<String>,
    pub estimated_complexity: u8,
    pub dependencies: Vec<String>,
}

/// Complete task plan with subtasks and assignments
#[derive(Debug, Clone)]
pub struct TaskPlan {
    pub objective: String,
    pub subtasks: Vec<Subtask>,
    pub agent_assignments: HashMap<String, Vec<String>>,
    pub dependencies: Vec<(String, String)>,
}

/// Task planner that decomposes objectives into subtasks
pub struct TaskPlanner {
    /// Agent responsible for task decomposition
    pub coordinator_agent: AgentProfile,
}

impl TaskPlanner {
    /// Create a new task planner
    pub fn new(coordinator_agent: AgentProfile) -> Self {
        Self { coordinator_agent }
    }

    /// Decompose an objective into a structured task plan
    pub async fn decompose_task(
        &self,
        objective: &str,
        available_agents: &[AgentProfile],
    ) -> Result<TaskPlan> {
        // TODO: Implement intelligent task decomposition using the coordinator agent
        // This would involve:
        // 1. Analyzing the objective to identify key components
        // 2. Breaking it down into manageable subtasks
        // 3. Identifying dependencies between subtasks
        // 4. Assigning subtasks to appropriate agents based on capabilities

        // For now, return a simple placeholder implementation
        let subtasks = self.create_placeholder_subtasks(objective);
        let agent_assignments = self.assign_tasks_to_agents(&subtasks, available_agents);
        let dependencies = self.identify_dependencies(&subtasks);

        Ok(TaskPlan {
            objective: objective.to_string(),
            subtasks,
            agent_assignments,
            dependencies,
        })
    }

    /// Create placeholder subtasks for demonstration
    fn create_placeholder_subtasks(&self, objective: &str) -> Vec<Subtask> {
        // This is a simplified implementation
        // In a real system, this would use the coordinator agent to intelligently
        // decompose the objective based on the available agents' capabilities

        vec![
            Subtask {
                id: "analysis".to_string(),
                description: format!("Analyze requirements for: {objective}"),
                required_capabilities: vec!["analysis".to_string(), "planning".to_string()],
                estimated_complexity: 3,
                dependencies: vec![],
            },
            Subtask {
                id: "design".to_string(),
                description: format!("Design solution for: {objective}"),
                required_capabilities: vec!["design".to_string(), "architecture".to_string()],
                estimated_complexity: 5,
                dependencies: vec!["analysis".to_string()],
            },
            Subtask {
                id: "implementation".to_string(),
                description: format!("Implement solution for: {objective}"),
                required_capabilities: vec!["coding".to_string(), "implementation".to_string()],
                estimated_complexity: 7,
                dependencies: vec!["design".to_string()],
            },
            Subtask {
                id: "testing".to_string(),
                description: format!("Test solution for: {objective}"),
                required_capabilities: vec!["testing".to_string(), "qa".to_string()],
                estimated_complexity: 4,
                dependencies: vec!["implementation".to_string()],
            },
        ]
    }

    /// Assign tasks to agents based on capabilities
    fn assign_tasks_to_agents(
        &self,
        subtasks: &[Subtask],
        available_agents: &[AgentProfile],
    ) -> HashMap<String, Vec<String>> {
        let mut assignments: HashMap<String, Vec<String>> = HashMap::new();

        for subtask in subtasks {
            // Find the best agent for this subtask based on capabilities
            let best_agent = available_agents
                .iter()
                .filter(|agent| {
                    // Check if agent has all required capabilities
                    subtask
                        .required_capabilities
                        .iter()
                        .all(|capability| agent.capabilities.contains(capability))
                })
                .max_by_key(|agent| {
                    // Score based on capability match
                    agent.capabilities.len()
                });

            if let Some(agent) = best_agent {
                assignments
                    .entry(agent.name.clone())
                    .or_default()
                    .push(subtask.id.clone());
            } else {
                // If no perfect match, assign to coordinator as fallback
                assignments
                    .entry(self.coordinator_agent.name.clone())
                    .or_default()
                    .push(subtask.id.clone());
            }
        }

        assignments
    }

    /// Identify dependencies between subtasks
    fn identify_dependencies(&self, subtasks: &[Subtask]) -> Vec<(String, String)> {
        let mut dependencies = Vec::new();

        for subtask in subtasks {
            for dependency in &subtask.dependencies {
                dependencies.push((dependency.clone(), subtask.id.clone()));
            }
        }

        dependencies
    }

    /// Calculate task readiness based on dependencies
    pub fn get_ready_tasks(
        &self,
        task_plan: &TaskPlan,
        completed_tasks: &[String],
    ) -> Vec<Subtask> {
        task_plan
            .subtasks
            .iter()
            .filter(|subtask| {
                // Task is ready if all dependencies are completed
                subtask
                    .dependencies
                    .iter()
                    .all(|dep| completed_tasks.contains(dep))
            })
            .cloned()
            .collect()
    }

    /// Validate that a task plan is feasible with available agents
    pub fn validate_task_plan(
        &self,
        task_plan: &TaskPlan,
        available_agents: &[AgentProfile],
    ) -> Result<()> {
        for subtask in &task_plan.subtasks {
            let can_handle = available_agents.iter().any(|agent| {
                subtask
                    .required_capabilities
                    .iter()
                    .all(|capability| agent.capabilities.contains(capability))
            });

            if !can_handle {
                anyhow::bail!(
                    "No agent can handle subtask '{}' with capabilities: {:?}",
                    subtask.id,
                    subtask.required_capabilities
                );
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_agents() -> Vec<AgentProfile> {
        vec![
            AgentProfile {
                name: "analyst".to_string(),
                role: "Analyst".to_string(),
                capabilities: vec!["analysis".to_string(), "planning".to_string()],
                model_provider: "deepseek".to_string(),
                model: "deepseek-chat".to_string(),
                instructions: None,
            },
            AgentProfile {
                name: "developer".to_string(),
                role: "Developer".to_string(),
                capabilities: vec![
                    "design".to_string(),
                    "coding".to_string(),
                    "implementation".to_string(),
                ],
                model_provider: "deepseek".to_string(),
                model: "deepseek-coder".to_string(),
                instructions: None,
            },
            AgentProfile {
                name: "tester".to_string(),
                role: "Tester".to_string(),
                capabilities: vec!["testing".to_string(), "qa".to_string()],
                model_provider: "openai".to_string(),
                model: "gpt-4o".to_string(),
                instructions: None,
            },
        ]
    }

    #[tokio::test]
    async fn test_task_decomposition() {
        let coordinator = AgentProfile {
            name: "coordinator".to_string(),
            role: "Coordinator".to_string(),
            capabilities: vec!["planning".to_string(), "coordination".to_string()],
            model_provider: "openai".to_string(),
            model: "gpt-4o".to_string(),
            instructions: None,
        };

        let planner = TaskPlanner::new(coordinator);
        let agents = create_test_agents();
        let objective = "Build a web application with user authentication";

        let task_plan = planner.decompose_task(objective, &agents).await.unwrap();

        assert_eq!(task_plan.objective, objective);
        assert!(!task_plan.subtasks.is_empty());
        assert!(!task_plan.agent_assignments.is_empty());
    }

    #[test]
    fn test_task_assignment() {
        let coordinator = AgentProfile {
            name: "coordinator".to_string(),
            role: "Coordinator".to_string(),
            capabilities: vec!["planning".to_string(), "coordination".to_string()],
            model_provider: "openai".to_string(),
            model: "gpt-4o".to_string(),
            instructions: None,
        };

        let planner = TaskPlanner::new(coordinator);
        let agents = create_test_agents();
        let subtasks = planner.create_placeholder_subtasks("test objective");

        let assignments = planner.assign_tasks_to_agents(&subtasks, &agents);

        // Check that tasks are assigned to appropriate agents
        assert!(assignments.contains_key("analyst"));
        assert!(assignments.contains_key("developer"));
        assert!(assignments.contains_key("tester"));
    }

    #[test]
    fn test_ready_tasks() {
        let coordinator = AgentProfile {
            name: "coordinator".to_string(),
            role: "Coordinator".to_string(),
            capabilities: vec!["planning".to_string(), "coordination".to_string()],
            model_provider: "openai".to_string(),
            model: "gpt-4o".to_string(),
            instructions: None,
        };

        let planner = TaskPlanner::new(coordinator);
        let task_plan = TaskPlan {
            objective: "test".to_string(),
            subtasks: vec![
                Subtask {
                    id: "task1".to_string(),
                    description: "Task 1".to_string(),
                    required_capabilities: vec!["cap1".to_string()],
                    estimated_complexity: 1,
                    dependencies: vec![],
                },
                Subtask {
                    id: "task2".to_string(),
                    description: "Task 2".to_string(),
                    required_capabilities: vec!["cap2".to_string()],
                    estimated_complexity: 1,
                    dependencies: vec!["task1".to_string()],
                },
            ],
            agent_assignments: HashMap::new(),
            dependencies: vec![],
        };

        // Initially, only task1 should be ready
        let ready_tasks = planner.get_ready_tasks(&task_plan, &[]);
        assert_eq!(ready_tasks.len(), 1);
        assert_eq!(ready_tasks[0].id, "task1");

        // After completing task1, task2 should be ready
        let ready_tasks = planner.get_ready_tasks(&task_plan, &["task1".to_string()]);
        assert_eq!(ready_tasks.len(), 1);
        assert_eq!(ready_tasks[0].id, "task2");
    }
}
