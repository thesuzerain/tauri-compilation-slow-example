//! Theseus state management system
use std::path::PathBuf;

use notify::RecommendedWatcher;
use notify_debouncer_mini::{new_debouncer, DebounceEventResult, Debouncer};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{OnceCell, RwLock, Semaphore};

use futures::{channel::mpsc::channel, SinkExt, StreamExt};

// Global state
static LAUNCHER_STATE: OnceCell<Arc<State>> = OnceCell::const_new();
pub struct State {
}

impl State {
    /// Get the current launcher state, initializing it if needed
    #[tracing::instrument]
    
    pub async fn get() -> crate::Result<Arc<Self>> {
        LAUNCHER_STATE
            .get_or_try_init(|| {
                async {
                    Ok(Arc::new(Self {
                    }))
                }
            })
            .await
            .map(Arc::clone)
    }
}
