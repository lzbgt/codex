//! Agent management and role taxonomy system
//!
//! This module provides dynamic agent creation based on LLM analysis
//! and standard role definitions for flexible multi-agent collaboration.

use anyhow::Context;
use anyhow::Result;
use anyhow::anyhow;
use codex_otel::otel_event_manager::OtelEventManager;
use codex_protocol::ConversationId;
use codex_protocol::config_types::ReasoningSummary as ReasoningSummaryConfig;
use codex_protocol::models::ContentItem;
use codex_protocol::models::ResponseItem;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::OnceLock;
use tracing::warn;
use uuid::Uuid;

use crate::auth::AuthManager;
use crate::client::ModelClient;
use crate::client_common::Prompt;
use crate::client_common::ResponseEvent;
use crate::config::Config;
use crate::model_family::derive_default_model_family;
use crate::model_provider_info::ModelProviderInfo;
use crate::model_provider_info::built_in_model_providers;

const DYNAMIC_ROLE_PLANNING_PROMPT: &str = include_str!("../../dynamic_role_planning_prompt.md");

fn dynamic_role_planning_instructions() -> &'static str {
    static PROMPT: OnceLock<String> = OnceLock::new();
    PROMPT.get_or_init(|| {
        let extracted = DYNAMIC_ROLE_PLANNING_PROMPT
            .split_once("```")
            .map(|x| x.1)
            .and_then(|rest| rest.split("```").next())
            .unwrap_or(DYNAMIC_ROLE_PLANNING_PROMPT);
        extracted.trim().to_string()
    })
}

fn extract_json_slice(text: &str) -> Option<&str> {
    let start = text.find('{')?;
    let end = text.rfind('}')?;
    if end >= start {
        Some(&text[start..=end])
    } else {
        None
    }
}

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
    /// Standard role identifier (e.g., "project_manager")
    pub standard_role: String,
    /// Default human-readable title
    pub default_title: String,
    /// Description of the role
    pub description: String,
    /// Capabilities that inform agent expertise
    pub capabilities: Vec<String>,
    /// Core competencies tied to standards
    pub core_competencies: Vec<String>,
    /// Typical responsibilities for fallback planning
    pub default_responsibilities: Vec<String>,
    /// Preferred model provider (if any)
    pub suggested_provider: Option<String>,
    /// Preferred model name (if any)
    pub suggested_model: Option<String>,
}

/// LLM-based role analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleAnalysis {
    /// Primary domain of the objective
    pub primary_domain: String,
    /// Primary standards referenced by the plan
    pub primary_standards: Vec<String>,
    /// Planned roles for this objective
    pub roles: Vec<RoleAssignment>,
    /// Task breakdown in ordered steps
    pub task_breakdown: Vec<String>,
    /// Risk register entries with mitigations
    pub risk_register: Vec<RiskRegisterEntry>,
    /// Optional complexity estimate (1-10)
    pub complexity_estimate: Option<u8>,
}

/// Role assignment from LLM analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleAssignment {
    /// Custom name for the role (e.g., "Web Delivery Lead")
    pub name: String,
    /// Standard role identifier from the allowed enumeration
    pub standard_role: String,
    /// Summary referencing governing standards
    pub summary: String,
    /// Core competencies aligned with the standards
    pub core_competencies: Vec<String>,
    /// Actionable responsibilities for the role
    pub responsibilities: Vec<String>,
}

/// Risk register entry with mitigation strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskRegisterEntry {
    pub risk: String,
    pub mitigation: String,
}

#[derive(Debug, Deserialize)]
struct LlmRolePlanningResponse {
    primary_domain: String,
    #[serde(default)]
    primary_standards: Vec<String>,
    #[serde(default)]
    roles: Vec<LlmRole>,
    #[serde(default)]
    task_breakdown: Vec<String>,
    #[serde(default)]
    risk_register: Vec<LlmRiskEntry>,
}

#[derive(Debug, Deserialize)]
struct LlmRole {
    name: String,
    standard_role: String,
    summary: String,
    #[serde(default)]
    core_competencies: Vec<String>,
    #[serde(default)]
    responsibilities: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct LlmRiskEntry {
    risk: String,
    mitigation: String,
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

        roles.insert(
            "project_manager".to_string(),
            RoleDefinition {
                standard_role: "project_manager".to_string(),
                default_title: "Project Manager".to_string(),
                description: "Coordinates planning and delivery using PMI PMBOK lifecycle controls"
                    .to_string(),
                capabilities: vec![
                    "project-planning".to_string(),
                    "risk-management".to_string(),
                    "stakeholder-alignment".to_string(),
                    "resource-allocation".to_string(),
                ],
                core_competencies: vec![
                    "Schedule Integration".to_string(),
                    "Risk Governance".to_string(),
                    "Stakeholder Communication".to_string(),
                ],
                default_responsibilities: vec![
                    "Define scope, schedule, and budget baselines".to_string(),
                    "Run risk reviews and maintain the RAID log".to_string(),
                    "Coordinate cross-functional status reporting".to_string(),
                ],
                suggested_provider: None,
                suggested_model: None,
            },
        );

        roles.insert(
            "technical_lead".to_string(),
            RoleDefinition {
                standard_role: "technical_lead".to_string(),
                default_title: "Technical Lead".to_string(),
                description:
                    "Guides solution implementation quality per software engineering standards"
                        .to_string(),
                capabilities: vec![
                    "architecture".to_string(),
                    "code-review".to_string(),
                    "technology-selection".to_string(),
                    "mentorship".to_string(),
                ],
                core_competencies: vec![
                    "Solution Governance".to_string(),
                    "Technical Risk Mitigation".to_string(),
                    "Quality Gates".to_string(),
                ],
                default_responsibilities: vec![
                    "Define technical approach and engineering guardrails".to_string(),
                    "Review implementation for compliance with standards".to_string(),
                    "Resolve complex engineering issues".to_string(),
                ],
                suggested_provider: None,
                suggested_model: None,
            },
        );

        roles.insert(
            "solution_architect".to_string(),
            RoleDefinition {
                standard_role: "solution_architect".to_string(),
                default_title: "Solution Architect".to_string(),
                description:
                    "Designs end-to-end systems aligned with enterprise architecture standards"
                        .to_string(),
                capabilities: vec![
                    "systems-thinking".to_string(),
                    "integration-design".to_string(),
                    "data-modeling".to_string(),
                    "security-architecture".to_string(),
                ],
                core_competencies: vec![
                    "Architecture Modeling".to_string(),
                    "Integration Strategy".to_string(),
                    "Security Controls".to_string(),
                ],
                default_responsibilities: vec![
                    "Create end-to-end architecture diagrams and guardrails".to_string(),
                    "Align solution with enterprise reference models".to_string(),
                    "Validate non-functional requirements coverage".to_string(),
                ],
                suggested_provider: None,
                suggested_model: None,
            },
        );

        roles.insert(
            "quality_engineer".to_string(),
            RoleDefinition {
                standard_role: "quality_engineer".to_string(),
                default_title: "Quality Engineer".to_string(),
                description: "Ensures outputs meet quality gates anchored in ISO/IEC 25010"
                    .to_string(),
                capabilities: vec![
                    "test-strategy".to_string(),
                    "automation".to_string(),
                    "defect-analysis".to_string(),
                    "compliance-testing".to_string(),
                ],
                core_competencies: vec![
                    "Test Planning".to_string(),
                    "Quality Metrics".to_string(),
                    "Continuous Improvement".to_string(),
                ],
                default_responsibilities: vec![
                    "Develop test plans aligned with acceptance criteria".to_string(),
                    "Execute validation cycles and log findings".to_string(),
                    "Report quality metrics and recommend improvements".to_string(),
                ],
                suggested_provider: None,
                suggested_model: None,
            },
        );

        roles.insert(
            "operations_engineer".to_string(),
            RoleDefinition {
                standard_role: "operations_engineer".to_string(),
                default_title: "Operations Engineer".to_string(),
                description: "Maintains reliability and incident readiness per ITIL 4 practices"
                    .to_string(),
                capabilities: vec![
                    "monitoring".to_string(),
                    "incident-response".to_string(),
                    "deployment".to_string(),
                    "capacity-planning".to_string(),
                ],
                core_competencies: vec![
                    "Service Operations".to_string(),
                    "SRE Practices".to_string(),
                    "Change Enablement".to_string(),
                ],
                default_responsibilities: vec![
                    "Define runbooks and monitoring thresholds".to_string(),
                    "Coordinate incident response drills".to_string(),
                    "Automate deployment and rollback safeguards".to_string(),
                ],
                suggested_provider: None,
                suggested_model: None,
            },
        );

        roles.insert(
            "product_designer".to_string(),
            RoleDefinition {
                standard_role: "product_designer".to_string(),
                default_title: "Product Designer".to_string(),
                description: "Leads human-centered design per ISO 9241-210 principles".to_string(),
                capabilities: vec![
                    "user-research".to_string(),
                    "interaction-design".to_string(),
                    "visual-communication".to_string(),
                    "prototyping".to_string(),
                ],
                core_competencies: vec![
                    "Design Thinking".to_string(),
                    "User Empathy".to_string(),
                    "Usability Evaluation".to_string(),
                ],
                default_responsibilities: vec![
                    "Facilitate discovery interviews and synthesize personas".to_string(),
                    "Prototype key workflows and validate usability".to_string(),
                    "Document design rationale and accessibility requirements".to_string(),
                ],
                suggested_provider: None,
                suggested_model: None,
            },
        );

        roles.insert(
            "data_scientist".to_string(),
            RoleDefinition {
                standard_role: "data_scientist".to_string(),
                default_title: "Data Scientist".to_string(),
                description: "Delivers analytics and models guided by CRISP-DM".to_string(),
                capabilities: vec![
                    "statistics".to_string(),
                    "machine-learning".to_string(),
                    "data-visualization".to_string(),
                    "experiment-design".to_string(),
                ],
                core_competencies: vec![
                    "Exploratory Analysis".to_string(),
                    "Model Development".to_string(),
                    "Evaluation Metrics".to_string(),
                ],
                default_responsibilities: vec![
                    "Profile datasets and document assumptions".to_string(),
                    "Build and validate predictive models".to_string(),
                    "Communicate insights with decision-ready visuals".to_string(),
                ],
                suggested_provider: None,
                suggested_model: None,
            },
        );

        roles.insert(
            "domain_expert".to_string(),
            RoleDefinition {
                standard_role: "domain_expert".to_string(),
                default_title: "Domain Expert".to_string(),
                description: "Supplies subject-matter knowledge against governing standards"
                    .to_string(),
                capabilities: vec![
                    "subject-knowledge".to_string(),
                    "policy-interpretation".to_string(),
                    "requirements-elicitation".to_string(),
                ],
                core_competencies: vec![
                    "Regulatory Context".to_string(),
                    "Stakeholder Insight".to_string(),
                    "Scenario Analysis".to_string(),
                ],
                default_responsibilities: vec![
                    "Clarify domain constraints and success measures".to_string(),
                    "Validate outputs against domain expectations".to_string(),
                    "Identify edge cases and compliance considerations".to_string(),
                ],
                suggested_provider: None,
                suggested_model: None,
            },
        );

        roles.insert(
            "human_reviewer".to_string(),
            RoleDefinition {
                standard_role: "human_reviewer".to_string(),
                default_title: "Human Reviewer".to_string(),
                description: "Provides qualitative assessment and sign-off per governance policy"
                    .to_string(),
                capabilities: vec![
                    "quality-assessment".to_string(),
                    "policy-compliance".to_string(),
                    "editorial-judgment".to_string(),
                ],
                core_competencies: vec![
                    "Policy Interpretation".to_string(),
                    "Editorial Standards".to_string(),
                    "Issue Escalation".to_string(),
                ],
                default_responsibilities: vec![
                    "Review deliverables for tone, accuracy, and compliance".to_string(),
                    "Capture feedback for iterative improvement".to_string(),
                    "Approve release readiness or flag blockers".to_string(),
                ],
                suggested_provider: None,
                suggested_model: None,
            },
        );

        roles.insert(
            "safety_officer".to_string(),
            RoleDefinition {
                standard_role: "safety_officer".to_string(),
                default_title: "Safety Officer".to_string(),
                description: "Oversees safety controls per OSHA 1910 and ISO 45001".to_string(),
                capabilities: vec![
                    "risk-assessment".to_string(),
                    "hazard-mitigation".to_string(),
                    "emergency-planning".to_string(),
                ],
                core_competencies: vec![
                    "Safety Auditing".to_string(),
                    "Incident Prevention".to_string(),
                    "Training Coordination".to_string(),
                ],
                default_responsibilities: vec![
                    "Conduct job hazard analysis and safety briefings".to_string(),
                    "Monitor control implementation and PPE readiness".to_string(),
                    "Coordinate incident response drills".to_string(),
                ],
                suggested_provider: None,
                suggested_model: None,
            },
        );

        roles.insert(
            "compliance_officer".to_string(),
            RoleDefinition {
                standard_role: "compliance_officer".to_string(),
                default_title: "Compliance Officer".to_string(),
                description: "Aligns outputs with regulatory frameworks such as ISO 37301"
                    .to_string(),
                capabilities: vec![
                    "regulatory-analysis".to_string(),
                    "audit-preparation".to_string(),
                    "control-testing".to_string(),
                ],
                core_competencies: vec![
                    "Policy Governance".to_string(),
                    "Evidence Management".to_string(),
                    "Third-party Oversight".to_string(),
                ],
                default_responsibilities: vec![
                    "Map deliverables to regulatory obligations".to_string(),
                    "Document evidence for audit readiness".to_string(),
                    "Coordinate remedial actions for control gaps".to_string(),
                ],
                suggested_provider: None,
                suggested_model: None,
            },
        );

        let mut domain_mappings = HashMap::new();
        domain_mappings.insert(
            "software_engineering".to_string(),
            vec!["technical_lead".to_string()],
        );
        domain_mappings.insert(
            "data_science".to_string(),
            vec!["data_scientist".to_string()],
        );
        domain_mappings.insert(
            "product_design".to_string(),
            vec!["product_designer".to_string()],
        );
        domain_mappings.insert(
            "documentation".to_string(),
            vec!["domain_expert".to_string()],
        );
        domain_mappings.insert(
            "business_strategy".to_string(),
            vec!["domain_expert".to_string()],
        );
        domain_mappings.insert(
            "operations".to_string(),
            vec!["operations_engineer".to_string()],
        );
        domain_mappings.insert(
            "infrastructure".to_string(),
            vec!["technical_lead".to_string()],
        );
        domain_mappings.insert(
            "compliance".to_string(),
            vec!["compliance_officer".to_string()],
        );
        domain_mappings.insert("education".to_string(), vec!["domain_expert".to_string()]);
        domain_mappings.insert("research".to_string(), vec!["domain_expert".to_string()]);
        domain_mappings.insert(
            "construction".to_string(),
            vec!["technical_lead".to_string(), "safety_officer".to_string()],
        );
        domain_mappings.insert("other".to_string(), vec!["technical_lead".to_string()]);

        RoleTaxonomy {
            roles,
            domain_mappings,
        }
    }

    /// Analyze an objective and determine required roles using LLM (with heuristic fallback)
    pub async fn analyze_objective(&self, objective: &str) -> Result<RoleAnalysis> {
        let trimmed = objective.trim();
        if trimmed.is_empty() {
            return Err(anyhow!("Objective must not be empty"));
        }

        // Tests and offline environments can set this to force heuristic fallback.
        if std::env::var("CODEX_DISABLE_ROLE_PLANNING_LLM").is_ok() {
            return Ok(self.fallback_role_analysis(trimmed));
        }

        match self.invoke_planning_llm(trimmed).await {
            Ok(raw) => match self.parse_planning_response(&raw, trimmed) {
                Ok(analysis) => Ok(self.finalize_analysis(trimmed, analysis)),
                Err(parse_err) => {
                    warn!(
                        "Failed to parse planning response. Falling back to heuristic plan: {parse_err:?}"
                    );
                    Ok(self.fallback_role_analysis(trimmed))
                }
            },
            Err(call_err) => {
                warn!("Planning LLM call failed. Falling back to heuristic plan: {call_err:?}");
                Ok(self.fallback_role_analysis(trimmed))
            }
        }
    }

    async fn invoke_planning_llm(&self, objective: &str) -> Result<String> {
        let provider = self.resolve_planning_provider()?;
        let planner_model = self.select_planning_model(&provider);

        let mut planning_config = self.config.clone();
        planning_config.model = planner_model.clone();
        planning_config.model_family = derive_default_model_family(&planner_model);
        planning_config.model_provider = provider.clone();
        planning_config.model_provider_id = self.config.model_provider_id.clone();
        planning_config
            .model_providers
            .entry(planning_config.model_provider_id.clone())
            .or_insert_with(|| provider.clone());

        let conversation_id = ConversationId::new();
        let otel_event_manager = OtelEventManager::new(
            conversation_id,
            planner_model.as_str(),
            "dynamic-role-planning",
            None,
            None,
            false,
            "multi-agent".to_string(),
        );

        let model_client = ModelClient::new(
            Arc::new(planning_config),
            Some(self.auth_manager.clone()),
            otel_event_manager,
            provider,
            None,
            ReasoningSummaryConfig::default(),
            conversation_id,
        );

        let instructions = dynamic_role_planning_instructions().to_string();
        let input = vec![
            ResponseItem::Message {
                id: None,
                role: "system".to_string(),
                content: vec![ContentItem::InputText { text: instructions }],
            },
            ResponseItem::Message {
                id: None,
                role: "user".to_string(),
                content: vec![ContentItem::InputText {
                    text: format!("Objective: {objective}"),
                }],
            },
        ];

        let prompt = Prompt {
            input,
            tools: Vec::new(),
            parallel_tool_calls: false,
            base_instructions_override: None,
            output_schema: None,
        };

        let mut response_stream = model_client.stream(&prompt).await?;
        let mut response_content = String::new();

        while let Some(event_result) = response_stream.rx_event.recv().await {
            match event_result {
                Ok(ResponseEvent::OutputTextDelta(delta)) => response_content.push_str(&delta),
                Ok(ResponseEvent::OutputItemDone(item)) => {
                    if let ResponseItem::Message { content, .. } = item {
                        for content_item in content {
                            if let ContentItem::OutputText { text } = content_item {
                                response_content.push_str(&text);
                            }
                        }
                    }
                }
                Ok(ResponseEvent::Completed { .. }) => break,
                Ok(_) => {}
                Err(e) => return Err(anyhow!("Model stream error: {e}")),
            }
        }

        let trimmed = response_content.trim();
        if trimmed.is_empty() {
            return Err(anyhow!(
                "Planning model returned an empty response for objective: {objective}"
            ));
        }
        Ok(trimmed.to_string())
    }

    fn parse_planning_response(&self, raw: &str, objective: &str) -> Result<RoleAnalysis> {
        let json_slice = extract_json_slice(raw).unwrap_or(raw);
        let parsed: LlmRolePlanningResponse = serde_json::from_str(json_slice)
            .or_else(|err| {
                let value: Value = serde_json::from_str(json_slice)?;
                serde_json::from_value(value).map_err(|_| err)
            })
            .context("failed to parse dynamic role planning response")?;

        let primary_domain = if parsed.primary_domain.trim().is_empty() {
            self.infer_domain(objective)
        } else {
            parsed.primary_domain
        };

        let mut primary_standards = if parsed.primary_standards.is_empty() {
            self.default_standards(&primary_domain)
        } else {
            parsed
                .primary_standards
                .into_iter()
                .filter(|s| !s.trim().is_empty())
                .collect()
        };
        if primary_standards.is_empty() {
            primary_standards = self.default_standards(&primary_domain);
        }

        let roles: Vec<RoleAssignment> = parsed
            .roles
            .into_iter()
            .map(|role| RoleAssignment {
                name: if role.name.trim().is_empty() {
                    role.standard_role.clone()
                } else {
                    role.name
                },
                standard_role: role.standard_role,
                summary: role.summary,
                core_competencies: role.core_competencies,
                responsibilities: role.responsibilities,
            })
            .collect();

        let mut risk_register: Vec<RiskRegisterEntry> = if parsed.risk_register.is_empty() {
            self.default_risks(&primary_domain, &primary_standards)
        } else {
            parsed
                .risk_register
                .into_iter()
                .filter(|entry| !entry.risk.trim().is_empty())
                .map(|entry| RiskRegisterEntry {
                    risk: entry.risk,
                    mitigation: entry.mitigation,
                })
                .collect()
        };
        if risk_register.len() > 2 {
            risk_register.truncate(2);
        }
        if risk_register.is_empty() {
            risk_register = self.default_risks(&primary_domain, &primary_standards);
        }

        let mut task_breakdown: Vec<String> = if parsed.task_breakdown.is_empty() {
            self.build_task_breakdown(objective, &roles, &primary_standards)
        } else {
            parsed
                .task_breakdown
                .into_iter()
                .filter(|step| !step.trim().is_empty())
                .collect()
        };
        if task_breakdown.len() < 3 {
            let mut fallback_steps =
                self.build_task_breakdown(objective, &roles, &primary_standards);
            task_breakdown.append(&mut fallback_steps);
            task_breakdown.dedup();
        }
        if task_breakdown.len() > 4 {
            task_breakdown.truncate(4);
        }

        let complexity_estimate =
            self.estimate_complexity(objective, roles.len(), task_breakdown.len());

        Ok(RoleAnalysis {
            primary_domain,
            primary_standards,
            roles,
            task_breakdown,
            risk_register,
            complexity_estimate,
        })
    }

    fn resolve_planning_provider(&self) -> Result<ModelProviderInfo> {
        if let Some(provider) = self
            .config
            .model_providers
            .get(&self.config.model_provider_id)
        {
            return Ok(provider.clone());
        }

        let built_in = built_in_model_providers();
        built_in
            .get(&self.config.model_provider_id)
            .cloned()
            .ok_or_else(|| {
                anyhow!(
                    "Unknown model provider id '{}' for role planning",
                    self.config.model_provider_id
                )
            })
    }

    fn select_planning_model(&self, provider: &ModelProviderInfo) -> String {
        if self.config.model_provider_id == "deepseek" {
            if self.config.model.starts_with("deepseek-") {
                self.config.model.clone()
            } else {
                "deepseek-reasoner".to_string()
            }
        } else if provider.name.eq_ignore_ascii_case("openai") {
            if self.config.model.is_empty() {
                "gpt-4o-mini".to_string()
            } else {
                self.config.model.clone()
            }
        } else if self.config.model.is_empty() {
            "gpt-4o-mini".to_string()
        } else {
            self.config.model.clone()
        }
    }

    fn fallback_role_analysis(&self, objective: &str) -> RoleAnalysis {
        let primary_domain = self.infer_domain(objective);
        let primary_standards = self.default_standards(&primary_domain);
        let roles = self.fallback_roles_for_domain(&primary_domain);
        let task_breakdown = self.build_task_breakdown(objective, &roles, &primary_standards);
        let risk_register = self.default_risks(&primary_domain, &primary_standards);
        let complexity_estimate =
            self.estimate_complexity(objective, roles.len(), task_breakdown.len());

        let analysis = RoleAnalysis {
            primary_domain,
            primary_standards,
            roles,
            task_breakdown,
            risk_register,
            complexity_estimate,
        };

        self.finalize_analysis(objective, analysis)
    }

    fn infer_domain(&self, objective: &str) -> String {
        let o = objective.to_lowercase();

        if o.contains("software")
            || o.contains("code")
            || o.contains("python")
            || o.contains("rust")
            || o.contains("script")
            || o.contains("function")
            || o.contains("program")
            || o.contains("app")
            || o.contains("backend")
            || o.contains("frontend")
            || o.contains("web")
            || o.contains("platform")
            || o.contains("system")
            || o.contains("service")
        {
            "software_engineering".to_string()
        } else if o.contains("data")
            || o.contains("model")
            || o.contains("analytics")
            || o.contains("statistic")
        {
            "data_science".to_string()
        } else if o.contains("design") || o.contains("ux") || o.contains("ui") {
            "product_design".to_string()
        } else if o.contains("deploy")
            || o.contains("runbook")
            || o.contains("monitor")
            || o.contains("ops")
        {
            "operations".to_string()
        } else if o.contains("infra") || o.contains("cloud") || o.contains("network") {
            "infrastructure".to_string()
        } else if o.contains("policy") || o.contains("compliance") || o.contains("audit") {
            "compliance".to_string()
        } else if o.contains("doc") || o.contains("manual") || o.contains("write") {
            "documentation".to_string()
        } else if o.contains("teach") || o.contains("curriculum") || o.contains("training") {
            "education".to_string()
        } else if o.contains("research") || o.contains("experiment") || o.contains("hypothesis") {
            "research".to_string()
        } else if o.contains("business") || o.contains("strategy") || o.contains("plan") {
            "business_strategy".to_string()
        } else {
            "other".to_string()
        }
    }

    fn default_standards(&self, domain: &str) -> Vec<String> {
        match domain {
            "software_engineering" => vec!["PMI PMBOK".to_string(), "ISO/IEC 12207".to_string()],
            "data_science" => vec!["PMI PMBOK".to_string(), "CRISP-DM".to_string()],
            "product_design" => vec!["PMI PMBOK".to_string(), "ISO 9241-210".to_string()],
            "documentation" => vec!["PMI PMBOK".to_string(), "ISO/IEC 82079".to_string()],
            "business_strategy" => vec!["PMI PMBOK".to_string(), "Balanced Scorecard".to_string()],
            "operations" => vec!["PMI PMBOK".to_string(), "ITIL 4".to_string()],
            "infrastructure" => vec!["PMI PMBOK".to_string(), "ISO/IEC 27001".to_string()],
            "compliance" => vec!["PMI PMBOK".to_string(), "ISO 37301".to_string()],
            "education" => vec!["PMI PMBOK".to_string(), "ISTE Standards".to_string()],
            "research" => vec![
                "PMI PMBOK".to_string(),
                "OECD Research Integrity".to_string(),
            ],
            _ => vec!["PMI PMBOK".to_string()],
        }
    }

    fn default_risks(&self, domain: &str, standards: &[String]) -> Vec<RiskRegisterEntry> {
        let primary = standards
            .first()
            .cloned()
            .unwrap_or_else(|| "PMI PMBOK".to_string());
        let secondary = standards.get(1).cloned().unwrap_or_else(|| primary.clone());

        match domain {
            "software_engineering" => vec![
                RiskRegisterEntry {
                    risk: "Scope creep impacting delivery cadence".to_string(),
                    mitigation: format!(
                        "Apply {primary} change control board review to approve scope changes"
                    ),
                },
                RiskRegisterEntry {
                    risk: "Quality regressions against ISO/IEC 12207 controls".to_string(),
                    mitigation: "Run gated quality reviews and automated regression suites"
                        .to_string(),
                },
            ],
            "data_science" => vec![
                RiskRegisterEntry {
                    risk: "Models drift due to insufficient validation".to_string(),
                    mitigation: format!(
                        "Follow CRISP-DM evaluation checkpoints and document metrics per {primary}"
                    ),
                },
                RiskRegisterEntry {
                    risk: "Data privacy non-compliance".to_string(),
                    mitigation: "Apply data governance policies and anonymization procedures"
                        .to_string(),
                },
            ],
            "product_design" => vec![
                RiskRegisterEntry {
                    risk: "Insufficient user empathy leading to poor adoption".to_string(),
                    mitigation:
                        "Run ISO 9241-210 informed research sprints and synthesize personas"
                            .to_string(),
                },
                RiskRegisterEntry {
                    risk: "Accessibility gaps against WCAG expectations".to_string(),
                    mitigation: "Conduct accessibility audits and remediate before launch"
                        .to_string(),
                },
            ],
            "operations" => vec![
                RiskRegisterEntry {
                    risk: "Incident response delays".to_string(),
                    mitigation: "Establish ITIL 4 major incident playbooks with on-call rotations"
                        .to_string(),
                },
                RiskRegisterEntry {
                    risk: "Safety hazards during change deployment".to_string(),
                    mitigation: "Implement change enablement with pre-deployment safety checks"
                        .to_string(),
                },
            ],
            "infrastructure" => vec![
                RiskRegisterEntry {
                    risk: "Configuration drifts jeopardizing security".to_string(),
                    mitigation: format!(
                        "Enforce {secondary} controls with infrastructure-as-code baselines"
                    ),
                },
                RiskRegisterEntry {
                    risk: "Capacity shortfalls under peak load".to_string(),
                    mitigation:
                        "Run capacity planning and stress testing with documented thresholds"
                            .to_string(),
                },
            ],
            "compliance" => vec![
                RiskRegisterEntry {
                    risk: "Regulatory obligations misinterpreted".to_string(),
                    mitigation: format!(
                        "Cross-check interpretations with {secondary} guidance and legal review"
                    ),
                },
                RiskRegisterEntry {
                    risk: "Audit evidence gaps".to_string(),
                    mitigation: "Maintain central evidence repository with traceable controls"
                        .to_string(),
                },
            ],
            _ => vec![
                RiskRegisterEntry {
                    risk: "Unclear requirements impacting delivery".to_string(),
                    mitigation: format!(
                        "Follow {primary} stakeholder alignment checkpoints to confirm scope"
                    ),
                },
                RiskRegisterEntry {
                    risk: "Schedule slippage".to_string(),
                    mitigation: "Establish cadence-based reviews and adjust plan proactively"
                        .to_string(),
                },
            ],
        }
    }

    fn fallback_roles_for_domain(&self, domain: &str) -> Vec<RoleAssignment> {
        let role_keys = self
            .taxonomy
            .domain_mappings
            .get(domain)
            .cloned()
            .unwrap_or_else(|| vec!["project_manager".to_string()]);

        role_keys
            .into_iter()
            .filter_map(|key| self.taxonomy.roles.get(&key))
            .take(4)
            .map(|definition| RoleAssignment {
                name: definition.default_title.clone(),
                standard_role: definition.standard_role.clone(),
                summary: definition.description.clone(),
                core_competencies: definition.core_competencies.clone(),
                responsibilities: definition.default_responsibilities.clone(),
            })
            .collect()
    }

    fn build_task_breakdown(
        &self,
        objective: &str,
        roles: &[RoleAssignment],
        _standards: &[String],
    ) -> Vec<String> {
        let mut steps = Vec::new();
        steps.push(format!(
            "Clarify success criteria for \"{objective}\" and collect any missing constraints."
        ));

        if roles.is_empty() {
            steps.push(format!(
                "Execute the core work for \"{objective}\" and document the approach."
            ));
        } else {
            for role in roles.iter().take(2) {
                let responsibility = role
                    .responsibilities
                    .first()
                    .cloned()
                    .unwrap_or_else(|| role.summary.clone());
                steps.push(format!(
                    "{name} executes: {responsibility}",
                    name = role.name
                ));
            }
            if roles.len() > 2 {
                steps.push(
                    "Coordinate any remaining specialists to cover outstanding tasks.".to_string(),
                );
            }
        }

        steps.push(format!(
            "Validate the output for \"{objective}\" with quick checks and capture follow-up notes."
        ));

        if steps.len() < 3 {
            steps.push(format!(
                "Summarize deliverables and surface any blockers for \"{objective}\"."
            ));
        }

        steps.truncate(4);
        steps
    }

    fn finalize_analysis(&self, objective: &str, mut analysis: RoleAnalysis) -> RoleAnalysis {
        let objective_lower = objective.to_lowercase();
        let is_simple = self.is_simple_objective(&objective_lower);

        if is_simple {
            analysis.primary_domain = "software_engineering".to_string();
            analysis.primary_standards.clear();
            analysis.roles = vec![self.single_technical_lead_role()];
            analysis.task_breakdown = self.simple_task_breakdown(objective);
            analysis.risk_register.clear();
            analysis.complexity_estimate = Some(1);
            return analysis;
        }

        if analysis.roles.is_empty() {
            analysis.roles.push(self.single_technical_lead_role());
        }
        analysis.roles.truncate(3);

        if analysis.task_breakdown.is_empty() {
            analysis.task_breakdown = self.simple_task_breakdown(objective);
        } else {
            let has_verification_step = analysis.task_breakdown.iter().any(|step| {
                let lower = step.to_lowercase();
                ["verify", "test", "validate", "review"]
                    .iter()
                    .any(|hint| lower.contains(hint))
            });
            if !has_verification_step {
                analysis.task_breakdown.push(format!(
                    "Verify the output for \"{objective}\" by running automated checks or executing the deliverable."
                ));
            }
            analysis.task_breakdown.truncate(6);
        }

        analysis.risk_register.truncate(2);

        analysis
    }

    fn is_simple_objective(&self, objective_lower: &str) -> bool {
        let trimmed_len = objective_lower.trim().len();
        if trimmed_len > 160 {
            return false;
        }

        let complex_terms = [
            "project",
            "program",
            "architecture",
            "compliance",
            "policy",
            "audit",
            "deployment",
            "infrastructure",
            "roadmap",
            "multi-agent",
            "migration",
            "platform",
        ];
        if complex_terms
            .iter()
            .any(|term| objective_lower.contains(term))
        {
            return false;
        }

        let simple_terms = [
            "script",
            "function",
            "snippet",
            "utility",
            "quick",
            "simple",
            "add",
            "sum",
            "hello world",
            "print",
            "format",
            "convert",
            "rename",
            "calculate",
        ];

        simple_terms
            .iter()
            .any(|term| objective_lower.contains(term))
    }

    fn single_technical_lead_role(&self) -> RoleAssignment {
        if let Some(definition) = self.taxonomy.roles.get("technical_lead") {
            return RoleAssignment {
                name: definition.default_title.clone(),
                standard_role: definition.standard_role.clone(),
                summary: definition.description.clone(),
                core_competencies: definition.core_competencies.clone(),
                responsibilities: definition.default_responsibilities.clone(),
            };
        }

        RoleAssignment {
            name: "Technical Lead".to_string(),
            standard_role: "technical_lead".to_string(),
            summary: "Owns design, implementation, and validation for focused engineering tasks"
                .to_string(),
            core_competencies: vec![
                "Implementation".to_string(),
                "Testing".to_string(),
                "Documentation".to_string(),
            ],
            responsibilities: vec![
                "Clarify requirements with the requester".to_string(),
                "Implement the requested change".to_string(),
                "Verify the result and report status".to_string(),
            ],
        }
    }

    fn simple_task_breakdown(&self, objective: &str) -> Vec<String> {
        vec![
            format!(
                "Confirm input/output expectations for \"{objective}\" and capture any sample data."
            ),
            format!(
                "Implement the deliverable for \"{objective}\" using the available workspace tools."
            ),
            format!(
                "Verify the implementation for \"{objective}\" by running tests, commands, or scripts and summarize the outcome."
            ),
        ]
    }

    fn estimate_complexity(
        &self,
        objective: &str,
        role_count: usize,
        task_count: usize,
    ) -> Option<u8> {
        let mut score = (objective.len() / 80) as u8 + role_count as u8;
        if task_count > 4 {
            score = score.saturating_add(1);
        }
        score = score.clamp(1, 10);
        Some(score)
    }

    /// Create agent profiles from role analysis
    pub fn create_agents_from_analysis(&self, analysis: &RoleAnalysis) -> Vec<AgentProfile> {
        let mut agents = Vec::new();

        for role_assignment in &analysis.roles {
            let role_def = self.taxonomy.roles.get(&role_assignment.standard_role);
            let mut expertise: Vec<String> = role_assignment.core_competencies.clone();
            let mut model_provider = self.config.model_provider_id.clone();
            let mut model = self.config.model.clone();

            if let Some(def) = role_def {
                expertise.extend(def.capabilities.clone());
                if let Some(provider) = &def.suggested_provider {
                    model_provider = provider.clone();
                }
                if let Some(model_override) = &def.suggested_model {
                    model = model_override.clone();
                }
            }

            let mut expertise_set: Vec<String> = {
                let set: HashSet<_> = expertise.into_iter().collect();
                set.into_iter().collect()
            };
            expertise_set.sort();

            let responsibilities = if role_assignment.responsibilities.is_empty() {
                "Coordinate with the team per referenced standards.".to_string()
            } else {
                role_assignment.responsibilities.join("; ")
            };

            let instructions = Some(format!(
                "You are serving as {name} ({standard_role}). {summary}. Focus on: {responsibilities}",
                name = role_assignment.name,
                standard_role = role_assignment.standard_role,
                summary = role_assignment.summary,
                responsibilities = responsibilities
            ));

            agents.push(AgentProfile {
                name: format!(
                    "{}-{}",
                    role_assignment.name.to_lowercase().replace(' ', "-"),
                    &Uuid::new_v4().to_string()[..8]
                ),
                role: role_assignment.name.clone(),
                capabilities: expertise_set,
                model_provider,
                model,
                instructions,
            });
        }

        agents
    }

    /// Get role definition by standard role name
    pub fn get_role_definition(&self, standard_role: &str) -> Option<&RoleDefinition> {
        self.taxonomy.roles.get(standard_role)
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

    fn test_config() -> Config {
        Config::load_from_base_config_with_overrides(
            crate::config::ConfigToml::default(),
            crate::config::ConfigOverrides::default(),
            std::path::PathBuf::from("."),
        )
        .unwrap()
    }

    #[test]
    fn test_role_taxonomy_creation() {
        let manager = AgentManager::new(test_config());
        assert!(manager.taxonomy.roles.contains_key("project_manager"));
        assert!(manager.taxonomy.roles.contains_key("technical_lead"));
        assert!(
            manager
                .taxonomy
                .domain_mappings
                .contains_key("software_engineering")
        );
    }

    #[test]
    fn test_role_definition_content() {
        let manager = AgentManager::new(test_config());
        let pm = manager.get_role_definition("project_manager").unwrap();
        assert_eq!(pm.default_title, "Project Manager");
        assert!(pm.capabilities.contains(&"project-planning".to_string()));
        assert!(
            pm.default_responsibilities
                .iter()
                .any(|r| r.contains("risk"))
        );
    }

    #[test]
    fn test_fallback_analysis_simple_objective_without_llm() {
        let manager = AgentManager::new(test_config());
        let analysis =
            manager.fallback_role_analysis("Write a small Python script that prints hello.");
        assert_eq!(analysis.primary_domain, "software_engineering");
        assert_eq!(analysis.roles.len(), 1);
        assert!(
            analysis
                .roles
                .iter()
                .all(|role| role.standard_role == "technical_lead")
        );
        assert!(analysis.task_breakdown.len() >= 3);
        assert!(analysis.task_breakdown.len() <= 4);
        assert!(analysis.risk_register.is_empty());
        assert!(analysis.complexity_estimate.is_some());
        assert!(
            analysis
                .task_breakdown
                .iter()
                .any(|step| step.to_lowercase().contains("verify"))
        );
    }
}
