// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod tsk_mgr;
mod fl_mgr;

fn main() {
    tsk_mgr::start_tsk_mgr();
    tauri::Builder::default()
        .invoke_handler(
            tauri::generate_handler![
                tsk_mgr::start_tsk_mgr,
                fl_mgr::display_disks,
                fl_mgr::open_disk,
                fl_mgr::open_dir,
                fl_mgr::search_file,
                fl_mgr::read_file
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}