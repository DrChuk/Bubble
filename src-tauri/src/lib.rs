// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri_plugin_positioner::{WindowExt, Position};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| {
            let window = tauri::WebviewWindowBuilder::new(
                app, 
                "Unito",
                tauri::WebviewUrl::App("index.html".into())
            )
            .title("Unito")
            .inner_size(400.0, 300.0)
            .decorations(false)
            .transparent(true)
            .skip_taskbar(true)
            .visible(false)
            .resizable(false)
            .build()?;

            #[cfg(target_os = "windows")]
            if let Err(e) = window.set_shadow(false) {
                eprintln!("Failed to disable shadow: {e}");
            }

            let _ = window.move_window(Position::TopCenter);

            let _ = window.show();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
