use dunce::canonicalize;
use futures::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::{collections::HashSet, path::Path};
use tempfile::NamedTempFile;
use tokio::task::JoinError;

use crate::state::State;

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub struct JavaVersion {
    pub path: String,
    pub version: String,
    pub architecture: String,
}
#[tracing::instrument]
pub async fn get_all_jre() -> Result<Vec<JavaVersion>, JREError> {
    // Use HashSet to avoid duplicates
    let mut jre_paths = HashSet::new();

    // Add JREs directly on PATH
    jre_paths.extend(get_all_jre_path().await?);
    jre_paths.extend(get_all_autoinstalled_jre_path().await?);

    panic!("hello");
}


// Gets all JREs from the PATH env variable
#[tracing::instrument]

async fn get_all_autoinstalled_jre_path() -> Result<HashSet<PathBuf>, JREError>{

    Box::pin(async move {
        let state = State::get().await.unwrap()  ;
        let mut jre_paths = HashSet::new();
        Ok(jre_paths)
    })
    .await
}

// Gets all JREs from the PATH env variable
#[tracing::instrument]
async fn get_all_jre_path() -> Result<HashSet<PathBuf>, JREError> {
    // Iterate over values in PATH variable, where accessible JREs are referenced
    let paths = env::var("PATH")?;
    Ok(env::split_paths(&paths).collect())
}

#[cfg(target_os = "windows")]
#[allow(dead_code)]
const JAVA_BIN: &str = "javaw.exe";

#[cfg(not(target_os = "windows"))]
#[allow(dead_code)]
const JAVA_BIN: &str = "java";


// For example filepath 'path', attempt to resolve it and get a Java version at this path
// If no such path exists, or no such valid java at this path exists, returns None
#[tracing::instrument]

pub async fn check_java_at_filepath(path: &Path) -> Option<JavaVersion> {
    // Attempt to canonicalize the potential java filepath
    // If it fails, this path does not exist and None is returned (no Java here)
    let Ok(path) = canonicalize(path) else { return None };

    // Checks for existence of Java at this filepath
    // Adds JAVA_BIN to the end of the path if it is not already there
    let java = if path.file_name()?.to_str()? != JAVA_BIN {
        path.join(JAVA_BIN)
    } else {
        path
    };

    if !java.exists() {
        return None;
    };

    let mut file = NamedTempFile::new().ok()?;
    file.write_all(include_bytes!("../../library/JavaInfo.class"))
        .ok()?;

    let original_path = file.path().to_path_buf();
    let mut new_path = original_path.clone();
    new_path.set_file_name("JavaInfo");
    new_path.set_extension("class");
    tokio::fs::rename(&original_path, &new_path).await.ok()?;

    // Run java checker on java binary
    let output = Command::new(&java)
        .arg("-cp")
        .arg(file.path().parent().unwrap())
        .arg("JavaInfo")
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut java_version = None;
    let mut java_arch = None;

    for line in stdout.lines() {
        let mut parts = line.split('=');
        let key = parts.next().unwrap_or_default();
        let value = parts.next().unwrap_or_default();

        if key == "os.arch" {
            java_arch = Some(value);
        } else if key == "java.version" {
            java_version = Some(value);
        }
    }

    // Extract version info from it
    if let Some(arch) = java_arch {
        if let Some(version) = java_version {
            let path = java.to_string_lossy().to_string();
            return Some(JavaVersion {
                path,
                version: version.to_string(),
                architecture: arch.to_string(),
            });
        }
    }
    None
}


#[derive(thiserror::Error, Debug)]
pub enum JREError {
    #[error("Command error : {0}")]
    IOError(#[from] std::io::Error),

    #[error("Env error: {0}")]
    EnvError(#[from] env::VarError),

    #[error("No JRE found for required version: {0}")]
    NoJREFound(String),

    #[error("Invalid JRE version string: {0}")]
    InvalidJREVersion(String),

    #[error("Parsing error: {0}")]
    ParseError(#[from] std::num::ParseIntError),

    #[error("Join error: {0}")]
    JoinError(#[from] JoinError),

    #[error("No stored tag for Minecraft Version {0}")]
    NoMinecraftVersionFound(String),

    #[error("Error getting launcher sttae")]
    StateError,
}
