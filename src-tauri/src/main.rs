// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_os = "macosx")]
#[tauri::command]
// Returns the marvel snap window position and size using the macosx api
fn find_window() -> (i32, i32, i32, i32) {
    return (0, 0, 0, 0);
}

#[cfg(target_os = "linux")]
#[tauri::command]
// Returns the marvel snap window position and size using the linux api
fn find_window() -> (i32, i32, i32, i32) {
    return (0, 0, 0, 0);
}
#[cfg(target_os = "windows")]
#[tauri::command]
// Returns the marvel snap window position and size using the windows api
fn find_window() -> (i32, i32, i32, i32) {
    use std::ptr::null;
    use windows_sys::{
        s,
        Win32::{
            Foundation::RECT,
            UI::WindowsAndMessaging::{FindWindowA, GetWindowRect},
        },
    };

    let window_handle = unsafe { FindWindowA(null(), s!("Snap")) };

    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    if window_handle != 0 && unsafe { GetWindowRect(window_handle, &mut rect) } != 0 {
        return (
            rect.left as i32,
            rect.right as i32,
            rect.top as i32,
            rect.bottom as i32,
        );
    }

    return (0, 0, 0, 0);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![find_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
