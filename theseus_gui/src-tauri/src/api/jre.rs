
use crate::api::Result;
use theseus::prelude::JavaVersion;
use theseus::prelude::*;

// Finds the highest version of Java 18+, if it exists
#[tauri::command]
pub async fn jre_find_jre_18plus_jres() -> Result<()> {
    Ok(jre::find_java18plus_jres().await?)
}

#[tauri::command]
pub async fn jre_find_jre_19plus_jres() -> Result<()> {
    Ok(jre::find_java19plus_jres().await?)
}

#[tauri::command]
pub async fn jre_find_jre_20plus_jres() -> Result<()> {
    Ok(jre::find_java20plus_jres().await?)
}

#[tauri::command]
pub async fn jre_find_jre_21plus_jres() -> Result<()> {
    Ok(jre::find_java21plus_jres().await?)

}

#[tauri::command]
pub async fn jre_find_jre_22plus_jres() -> Result<()> {
    Ok(jre::find_java22plus_jres().await?)
}

#[tauri::command]
pub async fn jre_find_jre_23plus_jres() -> Result<()> {
    Ok(jre::find_java23plus_jres().await?)
}

#[tauri::command]
pub async fn jre_find_jre_24plus_jres() -> Result<()> {
    Ok(jre::find_java24plus_jres().await?)
}

#[tauri::command]
pub async fn jre_find_jre_25plus_jres() -> Result<()> {
    Ok(jre::find_java25plus_jres().await?)
}

#[tauri::command]
pub async fn jre_find_jre_26plus_jres() -> Result<()> {
    Ok(jre::find_java26plus_jres().await?)
}

#[tauri::command]
pub async fn jre_find_jre_27plus_jres() -> Result<()> {
    Ok(jre::find_java27plus_jres().await?)
}

#[tauri::command]
pub async fn jre_find_jre_28plus_jres() -> Result<()> {
    Ok(jre::find_java28plus_jres().await?)
}

#[tauri::command]
pub async fn jre_find_jre_29plus_jres() -> Result<()> {
    Ok(jre::find_java29plus_jres().await?)
}

#[tauri::command]
pub async fn jre_find_jre_30plus_jres() -> Result<()> {
    Ok(jre::find_java30plus_jres().await?)
}
