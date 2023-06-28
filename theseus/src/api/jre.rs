//! Authentication flow interface
use reqwest::Method;
use serde::Deserialize;
use std::path::PathBuf;

use crate::{
    state::JavaGlobals,
    util::jre::{self, extract_java_majorminor_version, JavaVersion},
     State,
};

pub const JAVA_8_KEY: &str = "JAVA_8";
pub const JAVA_17_KEY: &str = "JAVA_17";
pub const JAVA_18PLUS_KEY: &str = "JAVA_18PLUS";

// Autodetect JavaSettings default
// Make a guess for what the default Java global settings should be
pub async fn autodetect_java_globals() -> crate::Result<JavaGlobals> {
    let mut java_8 = find_java8_jres().await?;
    let mut java_17 = find_java17_jres().await?;
    // let mut java_18plus = find_java18plus_jres().await?;

    // Simply select last one found for initial guess
    let mut java_globals = JavaGlobals::new();
    if let Some(jre) = java_8.pop() {
        java_globals.insert(JAVA_8_KEY.to_string(), jre);
    }
    if let Some(jre) = java_17.pop() {
        java_globals.insert(JAVA_17_KEY.to_string(), jre);
    }
    // if let Some(jre) = java_18plus.pop() {
    //     java_globals.insert(JAVA_18PLUS_KEY.to_string(), jre);
    // }

    Ok(java_globals)
}

// Searches for jres on the system that are 1.18 or higher
pub async fn find_java18plus_jres() -> crate::Result<()> {
    let version = extract_java_majorminor_version("1.18")?;
    let jres = jre::get_all_jre().await?;
    // Filter out JREs that are not 1.17 or higher
    // Ok(jres
    //     .into_iter()
    //     .filter(|jre| {
    //         let jre_version = extract_java_majorminor_version(&jre.version);
    //         if let Ok(jre_version) = jre_version {
    //             jre_version >= version
    //         } else {
    //             false
    //         }
    //     });
    Ok(())
}

pub async fn find_java19plus_jres() -> crate::Result<()> {
    let _ = find_java18plus_jres().await?;
    Ok(())
}

pub async fn find_java20plus_jres() -> crate::Result<()> {
    let _ = find_java18plus_jres().await?;
    let _ = find_java19plus_jres().await?;
    Ok(())
}

pub async fn find_java21plus_jres() -> crate::Result<()> {
    let _ = find_java18plus_jres().await?;
    let _ = find_java19plus_jres().await?;
    let _ = find_java20plus_jres().await?;
    Ok(())
}

pub async fn find_java22plus_jres() -> crate::Result<()> {
    let _ = find_java18plus_jres().await?;
    let _ = find_java19plus_jres().await?;
    let _ = find_java20plus_jres().await?;
    let _ = find_java21plus_jres().await?;
    Ok(())
}

pub async fn find_java23plus_jres() -> crate::Result<()> {
    let _ = find_java18plus_jres().await?;
    let _ = find_java19plus_jres().await?;
    let _ = find_java20plus_jres().await?;
    let _ = find_java21plus_jres().await?;
    let _ = find_java22plus_jres().await?;
    Ok(())
}

pub async fn find_java24plus_jres() -> crate::Result<()> {
    let _ = find_java18plus_jres().await?;
    let _ = find_java19plus_jres().await?;
    let _ = find_java20plus_jres().await?;
    let _ = find_java21plus_jres().await?;
    let _ = find_java22plus_jres().await?;
    let _ = find_java23plus_jres().await?;
    Ok(())
}

pub async fn find_java25plus_jres() -> crate::Result<()> {
    let _ = find_java18plus_jres().await?;
    let _ = find_java19plus_jres().await?;
    let _ = find_java20plus_jres().await?;
    let _ = find_java21plus_jres().await?;
    let _ = find_java22plus_jres().await?;
    let _ = find_java23plus_jres().await?;
    let _ = find_java24plus_jres().await?;
    Ok(())
}

pub async fn find_java26plus_jres() -> crate::Result<()> {
    let _ = find_java18plus_jres().await?;
    let _ = find_java19plus_jres().await?;
    let _ = find_java20plus_jres().await?;
    let _ = find_java21plus_jres().await?;
    let _ = find_java22plus_jres().await?;
    let _ = find_java23plus_jres().await?;
    let _ = find_java24plus_jres().await?;
    let _ = find_java25plus_jres().await?;
    Ok(())
}

pub async fn find_java27plus_jres() -> crate::Result<()> {
    let _ = find_java18plus_jres().await?;
    let _ = find_java19plus_jres().await?;
    let _ = find_java20plus_jres().await?;
    let _ = find_java21plus_jres().await?;
    let _ = find_java22plus_jres().await?;
    let _ = find_java23plus_jres().await?;
    let _ = find_java24plus_jres().await?;
    let _ = find_java25plus_jres().await?;
    let _ = find_java26plus_jres().await?;
    Ok(())
}

pub async fn find_java28plus_jres() -> crate::Result<()> {
    let _ = find_java18plus_jres().await?;
    let _ = find_java19plus_jres().await?;
    let _ = find_java20plus_jres().await?;
    let _ = find_java21plus_jres().await?;
    let _ = find_java22plus_jres().await?;
    let _ = find_java23plus_jres().await?;
    let _ = find_java24plus_jres().await?;
    let _ = find_java25plus_jres().await?;
    let _ = find_java26plus_jres().await?;
    let _ = find_java27plus_jres().await?;
    Ok(())
}

pub async fn find_java29plus_jres() -> crate::Result<()> {
    let _ = find_java18plus_jres().await?;
    let _ = find_java19plus_jres().await?;
    let _ = find_java20plus_jres().await?;
    let _ = find_java21plus_jres().await?;
    let _ = find_java22plus_jres().await?;
    let _ = find_java23plus_jres().await?;
    let _ = find_java24plus_jres().await?;
    let _ = find_java25plus_jres().await?;
    let _ = find_java26plus_jres().await?;
    let _ = find_java27plus_jres().await?;
    let _ = find_java28plus_jres().await?;
    Ok(())
}

pub async fn find_java30plus_jres() -> crate::Result<()> {
    let _ = find_java18plus_jres().await?;
    let _ = find_java19plus_jres().await?;
    let _ = find_java20plus_jres().await?;
    let _ = find_java21plus_jres().await?;
    let _ = find_java22plus_jres().await?;
    let _ = find_java23plus_jres().await?;
    let _ = find_java24plus_jres().await?;
    let _ = find_java25plus_jres().await?;
    let _ = find_java26plus_jres().await?;
    let _ = find_java27plus_jres().await?;
    let _ = find_java28plus_jres().await?;
    let _ = find_java29plus_jres().await?;
    Ok(())
}

// Searches for jres on the system that are 1.8 exactly
pub async fn find_java8_jres() -> crate::Result<Vec<JavaVersion>> {
    let version = extract_java_majorminor_version("1.8")?;
    let jres = jre::get_all_jre().await?;

    // Filter out JREs that are not 1.8
    Ok(jres
        .into_iter()
        .filter(|jre| {
            let jre_version = extract_java_majorminor_version(&jre.version);
            if let Ok(jre_version) = jre_version {
                jre_version == version
            } else {
                false
            }
        })
        .collect())
}

// Searches for jres on the system that are 1.17 exactly
pub async fn find_java17_jres() -> crate::Result<Vec<JavaVersion>> {
    let version = extract_java_majorminor_version("1.17")?;
    let jres = jre::get_all_jre().await?;

    // Filter out JREs that are not 1.8
    Ok(jres
        .into_iter()
        .filter(|jre| {
            let jre_version = extract_java_majorminor_version(&jre.version);
            if let Ok(jre_version) = jre_version {
                jre_version == version
            } else {
                false
            }
        })
        .collect())
}


pub async fn auto_install_java(java_version: u32) -> crate::Result<PathBuf> {
    panic!("done");
}

// Get all JREs that exist on the system
pub async fn get_all_jre() -> crate::Result<Vec<JavaVersion>> {
    Ok(jre::get_all_jre().await?)
}

pub async fn validate_globals() -> crate::Result<bool> {
    let state = State::get().await?;
    Ok(true)
}

// Validates JRE at a given at a given path
pub async fn check_jre(path: PathBuf) -> crate::Result<Option<JavaVersion>> {
    Ok(jre::check_java_at_filepath(&path).await)
}

// Gets maximum memory in KiB.
pub async fn get_max_memory() -> crate::Result<u64> {
    Ok(sys_info::mem_info()
        .map_err(|_| {
            crate::Error::from(crate::ErrorKind::LauncherError(
                "Unable to get computer memory".to_string(),
            ))
        })?
        .total)
}
