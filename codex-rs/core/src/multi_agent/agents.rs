//! Agent management and role taxonomy system
//!
//! This module provides dynamic agent creation based on LLM analysis
//! and standard role definitions for flexible multi-agent collaboration.

use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

use crate::auth::AuthManager;
use crate::config::Config;

/// Agent profile for dynamic role planning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProfile {
    pub name: String,
    pub role: String,
    pub capabilities: Vec<String>,
    pub model_provider: String,
    pub model: String,
    pub instructions: Option<String>,
}

/// Standard role taxonomy for dynamic agent creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleTaxonomy {
    /// Standard role names and their capabilities
    pub roles: HashMap<String, RoleDefinition>,
    /// Domain-specific role mappings
    pub domain_mappings: HashMap<String, Vec<String>>,
}

/// Definition of a standard role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleDefinition {
    /// Role name (e.g., "backend-developer", "data-scientist")
    pub name: String,
    /// Description of the role
    pub description: String,
    /// Core capabilities for this role
    pub capabilities: Vec<String>,
    /// Suggested model provider for this role
    pub suggested_provider: String,
    /// Suggested model for this role
    pub suggested_model: String,
    /// Common tasks this role handles
    pub common_tasks: Vec<String>,
}

/// LLM-based role analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleAnalysis {
    /// Primary domain of the objective
    pub primary_domain: String,
    /// Primary framework used for role planning
    pub primary_framework: String,
    /// Required roles for this objective
    pub required_roles: Vec<RoleAssignment>,
    /// Suggested task breakdown
    pub suggested_tasks: Vec<String>,
    /// Complexity estimate (1-10)
    pub complexity_estimate: u8,
}

/// Role assignment from LLM analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleAssignment {
    /// Role name from taxonomy
    pub role_name: String,
    /// Customized description for this specific task
    pub customized_description: String,
    /// Priority level (1-3, where 1 is highest)
    pub priority: u8,
    /// Estimated effort (1-10)
    pub estimated_effort: u8,
}

/// Agent manager for dynamic role creation
pub struct AgentManager {
    /// Standard role taxonomy
    pub taxonomy: RoleTaxonomy,
    /// Configuration
    pub config: Config,
    /// Auth manager for model client
    pub auth_manager: Arc<AuthManager>,
}

impl AgentManager {
    /// Create a new agent manager with standard taxonomy
    pub fn new(config: Config) -> Self {
        // Initialize auth manager for model client
        let codex_home = dirs::home_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join(".codex");
        let auth_manager = Arc::new(AuthManager::new(codex_home, false));

        Self {
            taxonomy: Self::create_standard_taxonomy(),
            config,
            auth_manager,
        }
    }

    /// Create the standard role taxonomy
    fn create_standard_taxonomy() -> RoleTaxonomy {
        let mut roles = HashMap::new();
        let mut domain_mappings = HashMap::new();

        // Technical Development Roles
        roles.insert(
            "backend-developer".to_string(),
            RoleDefinition {
                name: "backend-developer".to_string(),
                description: "Develops server-side logic, APIs, and database interactions"
                    .to_string(),
                capabilities: vec![
                    "api-design".to_string(),
                    "database".to_string(),
                    "server-side".to_string(),
                    "authentication".to_string(),
                    "performance".to_string(),
                ],
                suggested_provider: "deepseek".to_string(),
                suggested_model: "deepseek-coder".to_string(),
                common_tasks: vec![
                    "Implement REST API endpoints".to_string(),
                    "Design database schema".to_string(),
                    "Create authentication system".to_string(),
                    "Optimize server performance".to_string(),
                ],
            },
        );

        roles.insert(
            "frontend-developer".to_string(),
            RoleDefinition {
                name: "frontend-developer".to_string(),
                description: "Creates user interfaces and client-side functionality".to_string(),
                capabilities: vec![
                    "ui-design".to_string(),
                    "javascript".to_string(),
                    "react".to_string(),
                    "css".to_string(),
                    "user-experience".to_string(),
                ],
                suggested_provider: "deepseek".to_string(),
                suggested_model: "deepseek-chat".to_string(),
                common_tasks: vec![
                    "Create responsive UI components".to_string(),
                    "Implement user interactions".to_string(),
                    "Optimize frontend performance".to_string(),
                    "Ensure cross-browser compatibility".to_string(),
                ],
            },
        );

        // Data & Analytics Roles
        roles.insert(
            "data-scientist".to_string(),
            RoleDefinition {
                name: "data-scientist".to_string(),
                description: "Analyzes data, builds models, and provides insights".to_string(),
                capabilities: vec![
                    "data-analysis".to_string(),
                    "machine-learning".to_string(),
                    "statistics".to_string(),
                    "python".to_string(),
                    "visualization".to_string(),
                ],
                suggested_provider: "deepseek".to_string(),
                suggested_model: "deepseek-reasoner".to_string(),
                common_tasks: vec![
                    "Analyze datasets for patterns".to_string(),
                    "Build predictive models".to_string(),
                    "Create data visualizations".to_string(),
                    "Generate insights from data".to_string(),
                ],
            },
        );

        // Content & Writing Roles
        roles.insert(
            "content-writer".to_string(),
            RoleDefinition {
                name: "content-writer".to_string(),
                description: "Creates written content, documentation, and copy".to_string(),
                capabilities: vec![
                    "writing".to_string(),
                    "editing".to_string(),
                    "documentation".to_string(),
                    "copywriting".to_string(),
                    "proofreading".to_string(),
                ],
                suggested_provider: "openai".to_string(),
                suggested_model: "gpt-4o".to_string(),
                common_tasks: vec![
                    "Write technical documentation".to_string(),
                    "Create marketing copy".to_string(),
                    "Edit and proofread content".to_string(),
                    "Structure information effectively".to_string(),
                ],
            },
        );

        roles.insert(
            "technical-writer".to_string(),
            RoleDefinition {
                name: "technical-writer".to_string(),
                description: "Specializes in technical documentation and user guides".to_string(),
                capabilities: vec![
                    "technical-documentation".to_string(),
                    "api-documentation".to_string(),
                    "user-guides".to_string(),
                    "tutorials".to_string(),
                    "code-examples".to_string(),
                ],
                suggested_provider: "openai".to_string(),
                suggested_model: "gpt-4o".to_string(),
                common_tasks: vec![
                    "Document API endpoints".to_string(),
                    "Create user manuals".to_string(),
                    "Write technical tutorials".to_string(),
                    "Generate code documentation".to_string(),
                ],
            },
        );

        // Quality & Testing Roles
        roles.insert(
            "qa-engineer".to_string(),
            RoleDefinition {
                name: "qa-engineer".to_string(),
                description: "Ensures software quality through testing and validation".to_string(),
                capabilities: vec![
                    "testing".to_string(),
                    "quality-assurance".to_string(),
                    "automation".to_string(),
                    "validation".to_string(),
                    "bug-tracking".to_string(),
                ],
                suggested_provider: "deepseek".to_string(),
                suggested_model: "deepseek-chat".to_string(),
                common_tasks: vec![
                    "Create test cases".to_string(),
                    "Run automated tests".to_string(),
                    "Validate functionality".to_string(),
                    "Report and track bugs".to_string(),
                ],
            },
        );

        // Security & Operations Roles
        roles.insert(
            "security-auditor".to_string(),
            RoleDefinition {
                name: "security-auditor".to_string(),
                description: "Reviews code and systems for security vulnerabilities".to_string(),
                capabilities: vec![
                    "security".to_string(),
                    "code-review".to_string(),
                    "vulnerability-assessment".to_string(),
                    "penetration-testing".to_string(),
                    "compliance".to_string(),
                ],
                suggested_provider: "deepseek".to_string(),
                suggested_model: "deepseek-coder".to_string(),
                common_tasks: vec![
                    "Review code for security issues".to_string(),
                    "Assess system vulnerabilities".to_string(),
                    "Recommend security improvements".to_string(),
                    "Ensure compliance with standards".to_string(),
                ],
            },
        );

        // Project Management Roles
        roles.insert(
            "project-coordinator".to_string(),
            RoleDefinition {
                name: "project-coordinator".to_string(),
                description: "Coordinates team efforts and tracks project progress".to_string(),
                capabilities: vec![
                    "coordination".to_string(),
                    "planning".to_string(),
                    "communication".to_string(),
                    "tracking".to_string(),
                    "delegation".to_string(),
                ],
                suggested_provider: "openai".to_string(),
                suggested_model: "gpt-4o".to_string(),
                common_tasks: vec![
                    "Coordinate team activities".to_string(),
                    "Track project milestones".to_string(),
                    "Facilitate communication".to_string(),
                    "Manage task dependencies".to_string(),
                ],
            },
        );

        // Domain-specific mappings
        domain_mappings.insert(
            "web-development".to_string(),
            vec![
                "backend-developer".to_string(),
                "frontend-developer".to_string(),
                "qa-engineer".to_string(),
                "project-coordinator".to_string(),
            ],
        );

        domain_mappings.insert(
            "data-analysis".to_string(),
            vec![
                "data-scientist".to_string(),
                "content-writer".to_string(),
                "project-coordinator".to_string(),
            ],
        );

        domain_mappings.insert(
            "documentation".to_string(),
            vec![
                "technical-writer".to_string(),
                "content-writer".to_string(),
                "project-coordinator".to_string(),
            ],
        );

        domain_mappings.insert(
            "resume-optimization".to_string(),
            vec![
                "content-writer".to_string(),
                "technical-writer".to_string(),
                "project-coordinator".to_string(),
            ],
        );

        domain_mappings.insert(
            "code-review".to_string(),
            vec![
                "security-auditor".to_string(),
                "qa-engineer".to_string(),
                "project-coordinator".to_string(),
            ],
        );

        RoleTaxonomy {
            roles,
            domain_mappings,
        }
    }

    /// Analyze an objective and determine required roles using LLM
    pub async fn analyze_objective(&self, objective: &str) -> Result<RoleAnalysis> {
        // For now, use rule-based analysis as a fallback
        // In a full implementation, this would use LLM to analyze the objective

        let objective_lower = objective.to_lowercase();
        let mut required_roles = Vec::new();
        let mut primary_domain = "general".to_string();

        // Determine primary domain
        if objective_lower.contains("web")
            || objective_lower.contains("app")
            || objective_lower.contains("frontend")
            || objective_lower.contains("backend")
        {
            primary_domain = "web-development".to_string();
        } else if objective_lower.contains("data")
            || objective_lower.contains("analyze")
            || objective_lower.contains("statistic")
            || objective_lower.contains("model")
        {
            primary_domain = "data-analysis".to_string();
        } else if objective_lower.contains("document")
            || objective_lower.contains("write")
            || objective_lower.contains("manual")
            || objective_lower.contains("guide")
        {
            primary_domain = "documentation".to_string();
        } else if objective_lower.contains("resume")
            || objective_lower.contains("cv")
            || objective_lower.contains("career")
            || objective_lower.contains("job")
        {
            primary_domain = "resume-optimization".to_string();
        } else if objective_lower.contains("review")
            || objective_lower.contains("audit")
            || objective_lower.contains("security")
            || objective_lower.contains("test")
        {
            primary_domain = "code-review".to_string();
        }

        // Get suggested roles for the domain
        if let Some(suggested_role_names) = self.taxonomy.domain_mappings.get(&primary_domain) {
            for (index, role_name) in suggested_role_names.iter().enumerate() {
                if let Some(role_def) = self.taxonomy.roles.get(role_name) {
                    required_roles.push(RoleAssignment {
                        role_name: role_name.clone(),
                        customized_description: format!(
                            "{} for: {}",
                            role_def.description, objective
                        ),
                        priority: if index == 0 { 1 } else { 2 },
                        estimated_effort: 5, // Default medium effort
                    });
                }
            }
        }

        // Always include a coordinator for complex tasks
        if required_roles.len() > 1 {
            required_roles.push(RoleAssignment {
                role_name: "project-coordinator".to_string(),
                customized_description: format!("Coordinate team efforts for: {objective}"),
                priority: 1,
                estimated_effort: 3,
            });
        }

        // Generate suggested tasks
        let suggested_tasks = self.generate_suggested_tasks(objective, &required_roles);

        // Estimate complexity based on number of roles and objective length
        let complexity_estimate = std::cmp::min(
            10,
            (objective.len() / 50) as u8 + required_roles.len() as u8,
        );

        let primary_framework = primary_domain.clone();

        Ok(RoleAnalysis {
            primary_domain,
            primary_framework,
            required_roles,
            suggested_tasks,
            complexity_estimate,
        })
    }

    /// Generate suggested tasks based on objective and roles
    fn generate_suggested_tasks(&self, objective: &str, roles: &[RoleAssignment]) -> Vec<String> {
        let mut tasks = Vec::new();

        // Always start with analysis
        tasks.push(format!("Analyze requirements for: {objective}"));

        // Add role-specific tasks
        for role in roles {
            if let Some(role_def) = self.taxonomy.roles.get(&role.role_name) {
                for common_task in &role_def.common_tasks {
                    tasks.push(format!("{}: {}", role.role_name, common_task));
                }
            }
        }

        // Always end with review
        tasks.push("Review and validate final output".to_string());

        tasks
    }

    /// Create agent profiles from role analysis
    pub fn create_agents_from_analysis(&self, analysis: &RoleAnalysis) -> Vec<AgentProfile> {
        let mut agents = Vec::new();

        for role_assignment in &analysis.required_roles {
            if let Some(role_def) = self.taxonomy.roles.get(&role_assignment.role_name) {
                let agent = AgentProfile {
                    name: format!(
                        "{}-{}",
                        role_assignment.role_name,
                        &uuid::Uuid::new_v4().to_string()[..8]
                    ),
                    role: role_assignment.customized_description.clone(),
                    capabilities: role_def.capabilities.clone(),
                    model_provider: role_def.suggested_provider.clone(),
                    model: role_def.suggested_model.clone(),
                    instructions: Some(format!(
                        "You are a {} specializing in {}. Focus on your area of expertise and collaborate with other agents.",
                        role_def.name,
                        role_def.capabilities.join(", ")
                    )),
                };
                agents.push(agent);
            }
        }

        agents
    }

    /// Get role definition by name
    pub fn get_role_definition(&self, role_name: &str) -> Option<&RoleDefinition> {
        self.taxonomy.roles.get(role_name)
    }

    /// List all available roles
    pub fn list_roles(&self) -> Vec<&RoleDefinition> {
        self.taxonomy.roles.values().collect()
    }

    /// Get roles by domain
    pub fn get_roles_by_domain(&self, domain: &str) -> Vec<&RoleDefinition> {
        if let Some(role_names) = self.taxonomy.domain_mappings.get(domain) {
            role_names
                .iter()
                .filter_map(|name| self.taxonomy.roles.get(name))
                .collect()
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_taxonomy_creation() {
        let config = Config::load_from_base_config_with_overrides(
            crate::config::ConfigToml::default(),
            crate::config::ConfigOverrides::default(),
            std::path::PathBuf::from("."),
        )
        .unwrap();
        let manager = AgentManager::new(config);

        assert!(manager.taxonomy.roles.contains_key("backend-developer"));
        assert!(manager.taxonomy.roles.contains_key("frontend-developer"));
        assert!(manager.taxonomy.roles.contains_key("data-scientist"));
        assert!(
            manager
                .taxonomy
                .domain_mappings
                .contains_key("web-development")
        );
    }

    #[test]
    fn test_role_definition_content() {
        let config = Config::load_from_base_config_with_overrides(
            crate::config::ConfigToml::default(),
            crate::config::ConfigOverrides::default(),
            std::path::PathBuf::from("."),
        )
        .unwrap();
        let manager = AgentManager::new(config);

        let backend_role = manager.get_role_definition("backend-developer").unwrap();
        assert_eq!(backend_role.name, "backend-developer");
        assert!(
            backend_role
                .capabilities
                .contains(&"api-design".to_string())
        );
        assert!(
            backend_role
                .common_tasks
                .contains(&"Implement REST API endpoints".to_string())
        );
    }
}
