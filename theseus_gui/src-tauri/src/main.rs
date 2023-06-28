#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod tasks;
mod state;


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::do_command_1,
            commands::do_command_2,
            commands::do_command_3,
            commands::do_command_4,
            commands::do_command_5,
            commands::do_command_6,
            commands::do_command_7,
            commands::do_command_8,
            commands::do_command_9,
            commands::do_command_10,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}