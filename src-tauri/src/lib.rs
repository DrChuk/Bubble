use tauri::{Manager, WebviewWindow, Window};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri_plugin_positioner::{WindowExt, Position};
use chrono::Local;

#[tauri::command]
fn get_mouse_pos(window: WebviewWindow) -> Result<(f64, f64), String> {
    let mouse = window.cursor_position().map_err(|e| e.to_string())?;
    let window_position = window.inner_position().map_err(|e| e.to_string())?;

    let x = mouse.x - window_position.x as f64;
    let y = mouse.y - window_position.y as f64;

    let scale = window.scale_factor().map_err(|e| e.to_string())?;
    Ok((x / scale, y / scale))
}

#[tauri::command]
fn set_ignore_cursor_events(window: Window, ignore: bool) -> Result<(), String> {
    window.set_ignore_cursor_events(ignore).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_time() -> String {
    let time = Local::now();
    format!("{}", time.format("%H:%M"))
}

#[tauri::command]
fn get_date() -> String {
    let date = Local::now();
    format!("{}", date.format("%a %d %b"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| {
            let window = app.get_webview_window("bubble").unwrap();

            let window_clone = window.clone();
            tauri::async_runtime::spawn(async move {
                #[cfg(target_os = "windows")]
                if let Err(e) = window_clone.set_shadow(false) {
                    eprintln!("Failed to disable shadow: {e}");
                }

                let _ = window_clone.move_window(Position::TopCenter);
                let _ = window_clone.show();
            });


            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_mouse_pos,
            set_ignore_cursor_events,
            get_time,
            get_date
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
