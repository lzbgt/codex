//! Debug script for DeepSeek role planning

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
    ).unwrap();

    let agent_manager = AgentManager::new(config);

    // Test with a simple objective
    let objective = "Test DeepSeek integration";
    println!("\n📋 Testing with objective: {}", objective);

    // Analyze objective
    match agent_manager.analyze_objective(objective).await {
        Ok(analysis) => {
            println!("✅ Role analysis successful!");
            println!("Primary domain: {}", analysis.primary_domain);
            println!("Primary framework: {}", analysis.primary_framework);
            println!("Complexity estimate: {}", analysis.complexity_estimate);
            println!("Required roles: {}", analysis.required_roles.len());
            for role in &analysis.required_roles {
                println!("  - {} (priority: {}, effort: {})",
                    role.role_name, role.priority, role.estimated_effort);
            }
            println!("Suggested tasks: {}", analysis.suggested_tasks.len());
        }
        Err(e) => {
            println!("❌ Role analysis failed: {}", e);
            println!("Full error: {:?}", e);
        }
    }

    Ok(())
}