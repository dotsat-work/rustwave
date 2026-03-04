// src-tauri/src/lib.rs
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            if let Some(window) = app.get_webview_window("brainfm") {

                let _ = window.eval("
                    const style = document.createElement('style');
                    style.innerHTML = `
                        body, #root, .app-container, main {
                            background-color: transparent !important;
                        }
                    `;
                    document.head.appendChild(style);
                ");

                println!("Rustwave: Logged into window 'brainfm'");
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}