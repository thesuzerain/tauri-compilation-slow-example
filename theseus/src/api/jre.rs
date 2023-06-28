//! Authentication flow interface
use reqwest::Method;
use serde::Deserialize;
use std::path::PathBuf;

use crate::{
    util::{jre::{self, JavaVersion}, java_globals::JavaGlobals},
};

pub const JAVA_8_KEY: &str = "JAVA_8";
pub const JAVA_17_KEY: &str = "JAVA_17";
pub const JAVA_18PLUS_KEY: &str = "JAVA_18PLUS";


// Searches for jres on the system that are 1.18 or higher
pub async fn find_java18plus_jres() -> crate::Result<()> {
    let jres = jre::get_all_jre().await?;
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

// Get all JREs that exist on the system
pub async fn get_all_jre() -> crate::Result<Vec<JavaVersion>> {
    Ok(jre::get_all_jre().await?)
}

