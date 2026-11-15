use tauri::{Manager, WebviewWindow, Window};
use chrono::Local;
use tauri_plugin_positioner::{Position, WindowExt};

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
fn get_absolute_window_rect(window: WebviewWindow) -> Result<(i32, i32, i32, i32), String> {
    let window_position = window.inner_position().map_err(|e| e.to_string())?;
    let window_size = window.inner_size().map_err(|e| e.to_string())?;

    Ok((
        window_position.x,
        window_position.y,
        window_position.x + window_size.width as i32,
        window_position.y + window_size.height as i32,
    ))
}

#[tauri::command]
fn set_ignore_cursor_events(window: Window, ignore: bool) -> Result<(), String> {
    window
        .set_ignore_cursor_events(ignore)
        .map_err(|e| e.to_string())
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

#[tauri::command]
#[cfg(target_os = "windows")]
fn get_foreground_window_rect() -> Option<(i32, i32, i32, i32)> {
    use windows::Win32::Foundation::RECT;
    use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowRect};

    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.is_invalid() {
            return None;
        }
        let mut rect = RECT::default();
        if GetWindowRect(hwnd, &mut rect).is_ok() {
            Some((rect.left, rect.top, rect.right, rect.bottom))
        } else {
            None
        }
    }
}

#[tauri::command]
#[cfg(target_os = "windows")]
fn get_pixel(x: i32, y: i32) -> Option<(u8, u8, u8)> {
    use windows::Win32::Graphics::Gdi::{GetDC, GetPixel, ReleaseDC};
    use windows::Win32::Foundation::COLORREF;

    unsafe {
        let hdc = GetDC(None);
        if hdc.is_invalid() {
            return None;
        }
        let color: COLORREF = GetPixel(hdc, x, y);
        let _ = ReleaseDC(None, hdc);

        let r = (color.0 & 0xFF) as u8;
        let g = ((color.0 >> 8) & 0xFF) as u8;
        let b = ((color.0 >> 16) & 0xFF) as u8;

        Some((r, g, b))
    }
}

#[tauri::command]
fn is_dark(r: u8, g: u8, b: u8) -> bool {
    let brightness = (r as u32 * 299 + g as u32 * 587 + b as u32 * 114) / 1000;
    brightness < 10
}

#[tauri::command]
fn rects_intersect(a: (i32, i32, i32, i32), b: (i32, i32, i32, i32)) -> bool {
    !(a.2 < b.0 || b.2 < a.0 || a.3 < b.1 || b.3 < a.1)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("bubble") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
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

                #[cfg(debug_assertions)] {
                    let _ = window_clone.open_devtools();
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_mouse_pos,
            set_ignore_cursor_events,
            get_time,
            get_date,
            get_foreground_window_rect,
            rects_intersect,
            get_absolute_window_rect,
            get_pixel,
            is_dark
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
