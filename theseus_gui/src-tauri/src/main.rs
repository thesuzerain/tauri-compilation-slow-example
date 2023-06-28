#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod api;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            api::jre::jre_find_jre_18plus_jres,
            api::jre::jre_find_jre_19plus_jres,
            api::jre::jre_find_jre_20plus_jres,
            api::jre::jre_find_jre_21plus_jres,
            api::jre::jre_find_jre_22plus_jres,
            api::jre::jre_find_jre_23plus_jres,
            api::jre::jre_find_jre_24plus_jres,
            api::jre::jre_find_jre_25plus_jres,
            api::jre::jre_find_jre_26plus_jres,
            api::jre::jre_find_jre_27plus_jres,
            api::jre::jre_find_jre_28plus_jres,
            api::jre::jre_find_jre_29plus_jres,
            api::jre::jre_find_jre_30plus_jres,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
