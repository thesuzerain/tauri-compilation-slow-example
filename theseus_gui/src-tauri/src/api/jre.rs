
use crate::api::Result;
use theseus::prelude::JavaVersion;
use theseus::prelude::*;

/// Get all JREs that exist on the system
#[tauri::command]
pub async fn jre_get_all_jre() -> Result<Vec<JavaVersion>> {
    Ok(jre::get_all_jre().await?)
}

// finds the installation of Java 17, if it exists
#[tauri::command]
pub async fn jre_find_jre_17_jres() -> Result<Vec<JavaVersion>> {
    Ok(jre::find_java17_jres().await?)
}

// Finds the highest version of Java 18+, if it exists
#[tauri::command]
pub async fn jre_find_jre_18plus_jres() -> Result<Vec<JavaVersion>> {
    Ok(jre::find_java18plus_jres().await?)
}

#[tauri::command]
pub async fn jre_find_jre_19plus_jres() -> Result<Vec<JavaVersion>> {
    Ok(jre_find_jre_18plus_jres().await?)
}

// #[tauri::command]
// pub async fn jre_find_jre_20plus_jres() -> Result<()> {
//     Ok(jre::find_java20plus_jres().await?)
// }

// #[tauri::command]
// pub async fn jre_find_jre_21plus_jres() -> Result<()> {
//     Ok(jre::find_java21plus_jres().await?)

// }

// #[tauri::command]
// pub async fn jre_find_jre_22plus_jres() -> Result<()> {
//     Ok(jre::find_java22plus_jres().await?)
// }

// #[tauri::command]
// pub async fn jre_find_jre_23plus_jres() -> Result<()> {
//     Ok(jre::find_java23plus_jres().await?)
// }

// #[tauri::command]
// pub async fn jre_find_jre_24plus_jres() -> Result<()> {
//     Ok(jre::find_java24plus_jres().await?)
// }

// #[tauri::command]
// pub async fn jre_find_jre_25plus_jres() -> Result<()> {
//     Ok(jre::find_java25plus_jres().await?)
// }