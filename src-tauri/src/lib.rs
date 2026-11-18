mod updater;

use tauri::{Emitter, Manager};
use tauri_plugin_fs::FsExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // fs 要先注册，对话框才能把用户选中的路径放进它的访问范围。
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            updater::schedule(app);
            let Some(window) = app.get_webview_window("main") else {
                return Ok(());
            };
            let handle = app.handle().clone();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::DragDrop(tauri::DragDropEvent::Drop { paths, .. }) =
                    event
                {
                    // 页面上的 HTML5 drop 在 Windows WebView2 里收不到文件。
                    // 这里只放开用户刚刚拖进来的路径，再通知界面去读，避免先读后授权。
                    if let Some(scope) = handle.try_fs_scope() {
                        for path in paths {
                            if path.is_dir() {
                                let _ = scope.allow_directory(path, true);
                            } else if path.is_file() {
                                let _ = scope.allow_file(path);
                            }
                        }
                    }
                    let _ = handle.emit("deepic://paths-ready", paths);
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
