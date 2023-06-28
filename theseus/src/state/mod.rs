//! Theseus state management system
use std::path::PathBuf;

use notify::RecommendedWatcher;
use notify_debouncer_mini::{new_debouncer, DebounceEventResult, Debouncer};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{OnceCell, RwLock, Semaphore};

use futures::{channel::mpsc::channel, SinkExt, StreamExt};

// Submodules
mod dirs;
pub use self::dirs::*;

mod settings;
pub use self::settings::*;

mod java_globals;
pub use self::java_globals::*;

// Global state
static LAUNCHER_STATE: OnceCell<Arc<State>> = OnceCell::const_new();
pub struct State {
    /// Information on the location of files used in the launcher
    pub directories: DirectoryInfo,

    /// File watcher debouncer
    pub(crate) file_watcher: RwLock<Debouncer<RecommendedWatcher>>,
}

impl State {
    /// Get the current launcher state, initializing it if needed
    #[tracing::instrument]
    
    pub async fn get() -> crate::Result<Arc<Self>> {
        LAUNCHER_STATE
            .get_or_try_init(|| {
                async {

                    let mut file_watcher = init_watcher().await?;

                    let directories = DirectoryInfo::init().await?;

                    // Settings
                    let settings =
                        Settings::init(&directories.settings_file()).await?;



                    Ok(Arc::new(Self {
                        directories,
                        file_watcher: RwLock::new(file_watcher),
                    }))
                }
            })
            .await
            .map(Arc::clone)
    }

    /// Updates state with data from the web
    pub fn update() {
        tokio::task::spawn(Settings::update_java());
    }

    #[tracing::instrument]
    
    /// Synchronize in-memory state with persistent state
    pub async fn sync() -> crate::Result<()> {
        let state = Self::get().await?;
        let sync_settings = async {
            let state = Arc::clone(&state);

        };

        let sync_profiles = async {
            let state = Arc::clone(&state);

            tokio::spawn(async move {
                Ok::<_, crate::Error>(())
            })
            .await?
        };

        tokio::try_join!(sync_profiles)?;
        Ok(())
    }

    /// Reset IO semaphore to default values
    /// This will block until all uses of the semaphore are complete, so it should only be called
    /// when we are not in the middle of downloading something (ie: changing the settings!)
    pub async fn reset_io_semaphore(&self) {
    }

    /// Reset IO semaphore to default values
    /// This will block until all uses of the semaphore are complete, so it should only be called
    /// when we are not in the middle of downloading something (ie: changing the settings!)
    pub async fn reset_fetch_semaphore(&self) {

    }
}

async fn init_watcher() -> crate::Result<Debouncer<RecommendedWatcher>> {
    let (mut tx, mut rx) = channel(1);

    let file_watcher = new_debouncer(
        Duration::from_secs(2),
        None,
        move |res: DebounceEventResult| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
    )?;

    tokio::task::spawn(async move {
        while let Some(res) = rx.next().await {
            match res {
                Ok(events) => {
                    let mut visited_paths = Vec::new();
                    events.iter().for_each(|e| {
                        let mut new_path = PathBuf::new();
                        let mut found = false;

                        for component in e.path.components() {
                            new_path.push(component);
                            if found {
                                break;
                            }
                            if component.as_os_str() == "profiles" {
                                found = true;
                            }
                        }

                        if !visited_paths.contains(&new_path) {
                            visited_paths.push(new_path);
                        }
                    });
                }
                Err(errors) => errors.iter().for_each(|err| {
                    tracing::warn!("Unable to watch file: {err}")
                }),
            }
        }
    });

    Ok(file_watcher)
}
