//! Test script for the casual multi-agent collaboration system

use codex_core::multi_agent::casual::api;
use codex_core::config::Config;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🧪 Testing Casual Multi-Agent Collaboration System");

    // Load default configuration
    let config = Config::load_with_cli_overrides(
        codex_common::CliConfigOverrides::default().parse_overrides()?,
        codex_core::config::ConfigOverrides::default()
    ).await?;

    let config = Arc::new(config);

    // Initialize the system
    api::init(config.clone()).await?;
    println!("✅ System initialized successfully");

    // Test with a simple objective
    let objective = "Help improve my resume";
    println!("\n📋 Testing with objective: {}", objective);

    // Publish the task
    let session = api::publish_task(objective.to_string(), false).await?;
    println!("✅ Task published successfully");
    println!("Task ID: {}", session.task_id);

    // Get initial progress
    let snapshot = api::peek_at_progress(&session.task_id).await?;
    println!("\n📊 Initial Progress:");
    println!("  Status: {}", snapshot.status);
    println!("  Progress: {}%", snapshot.progress_percentage);
    println!("  Active Agents: {:?}", snapshot.active_agents);

    // Wait a bit for planning phase
    println!("\n⏳ Waiting for planning phase...");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Check progress again
    let snapshot = api::peek_at_progress(&session.task_id).await?;
    println!("\n📊 Progress after planning:");
    println!("  Status: {}", snapshot.status);
    println!("  Progress: {}%", snapshot.progress_percentage);
    println!("  Active Agents: {:?}", snapshot.active_agents);

    // Get detailed status
    let detailed_status = api::get_detailed_status(&session.task_id).await?;
    println!("\n📋 Detailed Status:");
    println!("  Objective: {}", detailed_status.objective);
    println!("  Overall Status: {}", detailed_status.overall_status);
    println!("  Agent Count: {}", detailed_status.agent_statuses.len());
    println!("  Artifact Count: {}", detailed_status.artifact_count);

    // Save session state
    api::save_session_state(&session.task_id).await?;
    println!("\n💾 Session state saved successfully");

    println!("\n🎉 Multi-agent system test completed successfully!");

    Ok(())
}