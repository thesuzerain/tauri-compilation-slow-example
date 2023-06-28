#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


mod mock_api;

// Should be called in launcher initialization
#[tauri::command]
async fn initialize_state(app: tauri::AppHandle) -> Result<(), ()> {
    Ok(())
}

fn main() {
    let mut builder = tauri::Builder::default().invoke_handler(tauri::generate_handler![
        initialize_state,
    ]);

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

