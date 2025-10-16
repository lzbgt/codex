//! Debug script for DeepSeek role planning

use anyhow::Context;
use codex_core::config::Config;
use codex_core::multi_agent::AgentManager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🧪 Debugging DeepSeek Role Planning");

    // Load default configuration
    let config = Config::load_from_base_config_with_overrides(
        codex_core::config::ConfigToml::default(),
        codex_core::config::ConfigOverrides::default(),
        std::path::PathBuf::from("."),
    )
    .context("failed to load default config overrides for debug role planning")?;

    let agent_manager = AgentManager::new(config);

    // Test with a simple objective
    let objective = "Test DeepSeek integration";
    println!("\n📋 Testing with objective: {}", objective);

    // Analyze objective
    match agent_manager.analyze_objective(objective).await {
        Ok(analysis) => {
            println!("✅ Role analysis successful!");
            println!("Primary domain: {}", analysis.primary_domain);
            if !analysis.primary_standards.is_empty() {
                println!("Primary standards: {}", analysis.primary_standards.join(", "));
            }
            if let Some(score) = analysis.complexity_estimate {
                println!("Complexity estimate: {score}/10");
            }
            println!("Planned roles: {}", analysis.roles.len());
            for role in &analysis.roles {
                let key_resp = role
                    .responsibilities
                    .first()
                    .cloned()
                    .unwrap_or_else(|| role.summary.clone());
                println!("  - {} [{}] -> {}", role.name, role.standard_role, key_resp);
            }
            println!("Task breakdown ({} steps):", analysis.task_breakdown.len());
            for (idx, step) in analysis.task_breakdown.iter().enumerate() {
                println!("    {}. {}", idx + 1, step);
            }
            println!("Risk register:");
            for entry in &analysis.risk_register {
                println!("    - {} => {}", entry.risk, entry.mitigation);
            }
        }
        Err(e) => {
            println!("❌ Role analysis failed: {}", e);
            println!("Full error: {:?}", e);
        }
    }

    Ok(())
}
