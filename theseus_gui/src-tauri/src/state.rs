//! Theseus state management system

use std::sync::Arc;
use tokio::sync::OnceCell;

// Global state
static LAUNCHER_STATE: OnceCell<Arc<State>> = OnceCell::const_new();
pub struct State {}

impl State {
    /// Get the current launcher state, initializing it if needed

    pub async fn get() -> Result<Arc<Self>,()> {
        LAUNCHER_STATE
            .get_or_try_init(|| async { Ok(Arc::new(Self {})) })
            .await
            .map(Arc::clone)
    }
}
