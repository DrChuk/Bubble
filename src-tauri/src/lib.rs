use tauri::{Manager, WebviewWindow, Window};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri_plugin_positioner::{WindowExt, Position};

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| {
            let window = app.get_webview_window("bubble").expect("Bubble not found!");

            #[cfg(target_os = "windows")]
            if let Err(e) = window.set_shadow(false) {
                eprintln!("Failed to disable shadow: {e}");
            }

            let _ = window.move_window(Position::TopCenter);

            let _ = window.show();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_mouse_pos, set_ignore_cursor_events])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
