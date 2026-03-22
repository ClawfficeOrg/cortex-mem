use axum::{
    Json,
    extract::{Path, State},
};
use std::sync::Arc;

use crate::{
    error::{AppError, Result},
    models::ApiResponse,
    state::AppState,
};

/// Trigger memory extraction for a session.
///
/// This endpoint is now a convenience wrapper over the standard session-close pipeline.
/// It marks the session as closed, runs memory extraction + L0/L1 generation synchronously
/// via `MemoryEventCoordinator`, and returns a summary of the extracted data.
///
/// Note: the `cortex-mem-service` REST layer does not hold a `MemoryEventCoordinator`
/// reference directly (it uses `CortexMem` which wires up the coordinator internally).
/// For now, this endpoint delegates to `SessionManager::close_session` which sends a
/// `SessionClosed` event that the coordinator handles asynchronously.
pub async fn trigger_extraction(
    State(state): State<Arc<AppState>>,
    Path(thread_id): Path<String>,
) -> Result<Json<ApiResponse<serde_json::Value>>> {
    // Ensure LLM is available (coordinator needs it)
    if state.llm_client.is_none() {
        return Err(AppError::BadRequest(
            "LLM client not configured. Set LLM_API_BASE_URL, LLM_API_KEY, and LLM_MODEL \
             environment variables."
                .to_string(),
        ));
    }

    // Close the session — this sends a SessionClosed event to MemoryEventCoordinator which
    // handles memory extraction, L0/L1 generation and vector sync asynchronously.
    let session_mgr = state.current_session_manager().await;
    let mut session_mgr = session_mgr.write().await;
    session_mgr.close_session(&thread_id).await?;

    let response = serde_json::json!({
        "thread_id": thread_id,
        "status": "extraction_triggered",
        "message": "Session closed. Memory extraction and L0/L1 generation are being processed \
                    asynchronously by MemoryEventCoordinator.",
    });

    Ok(Json(ApiResponse::success(response)))
}
