use anyhow::Result;
use colored::Colorize;
use cortex_mem_tools::MemoryOperations;
use std::sync::Arc;

pub async fn list(operations: Arc<MemoryOperations>) -> Result<()> {
    println!("{} Listing all sessions", "📋".bold());

    let sessions = operations.list_sessions().await?;

    if sessions.is_empty() {
        println!("\n{} No sessions found", "ℹ".yellow().bold());
        return Ok(());
    }

    println!("\n{} Found {} sessions:", "✓".green().bold(), sessions.len());
    println!();

    for session in sessions {
        println!("• {}", session.thread_id.bright_blue().bold());
        println!("  {}: {}", "Status".dimmed(), session.status);
        println!("  {}: {}", "Created".dimmed(), session.created_at.format("%Y-%m-%d %H:%M:%S UTC"));
        println!("  {}: {}", "Updated".dimmed(), session.updated_at.format("%Y-%m-%d %H:%M:%S UTC"));
        println!();
    }

    Ok(())
}

pub async fn create(
    operations: Arc<MemoryOperations>,
    thread: &str,
    title: Option<&str>,
) -> Result<()> {
    println!("{} Creating session: {}", "📝".bold(), thread.cyan());

    // Add a system message to create the session
    let message = if let Some(t) = title {
        format!("Session: {}", t)
    } else {
        "Session created".to_string()
    };
    
    operations.add_message(thread, "system", &message).await?;

    println!("{} Session created successfully", "✓".green().bold());
    println!("  {}: {}", "Thread ID".cyan(), thread);
    if let Some(t) = title {
        println!("  {}: {}", "Title".cyan(), t);
    }

    Ok(())
}

/// Close a session and synchronously wait for memory extraction, L0/L1 generation,
/// and vector indexing to complete before returning.
pub async fn close(operations: Arc<MemoryOperations>, thread: &str) -> Result<()> {
    println!("{} Closing session: {}", "🔒".bold(), thread.cyan());
    println!("{} Waiting for memory extraction, L0/L1 generation, and indexing to complete...", "⏳".yellow().bold());

    // close_session_sync blocks until the full pipeline completes:
    // 1. Session metadata → marked closed
    // 2. LLM memory extraction from session timeline
    // 3. user/agent memory files written
    // 4. L0/L1 layer files generated for all affected directories
    // 5. Session timeline synced to vector store
    operations.close_session_sync(thread).await?;

    println!("{} Session closed and all processing completed", "✓".green().bold());
    println!("  {}: {}", "Thread ID".cyan(), thread);

    Ok(())
}
