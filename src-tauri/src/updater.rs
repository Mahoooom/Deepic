//! 启动时检查 GitHub Releases 上的新版本。
//!
//! 只在正式包里检查。开发模式（`tauri dev`）直接跳过，避免每次打开都弹窗。

use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
use tauri_plugin_updater::UpdaterExt;

pub fn schedule(app: &tauri::App) {
    if cfg!(debug_assertions) {
        return;
    }

    let handle = app.handle().clone();
    tauri::async_runtime::spawn(async move {
        if let Err(err) = check_and_prompt(handle).await {
            eprintln!("deepic updater: {err}");
        }
    });
}

async fn check_and_prompt(app: AppHandle) -> Result<(), String> {
    let update = match app.updater() {
        Ok(updater) => match updater.check().await {
            Ok(Some(update)) => update,
            Ok(None) => return Ok(()),
            Err(err) => {
                eprintln!("deepic updater: check failed: {err}");
                return Ok(());
            }
        },
        Err(err) => {
            eprintln!("deepic updater: init failed: {err}");
            return Ok(());
        }
    };

    let version = update.version.clone();
    let current = update.current_version.clone();
    let mut message = format!("发现新版本 {version}（当前是 {current}）。");
    if let Some(notes) = update
        .body
        .as_deref()
        .map(str::trim)
        .filter(|notes| !notes.is_empty())
        .map(shorten_notes)
    {
        message.push_str("\n\n");
        message.push_str(&notes);
    }
    message.push_str("\n\n点「立即更新」后会下载并安装，完成后自动重启。");

    if !ask(&app, "发现新版本", message, Ask::Confirm).await {
        return Ok(());
    }

    let _ = app.emit(
        "deepic://update-status",
        format!("正在下载并安装 {version}，请稍候…"),
    );

    if let Err(err) = update.download_and_install(|_, _| {}, || {}).await {
        let _ = ask(
            &app,
            "更新失败",
            format!(
                "更新没有完成。\n\n{err}\n\n可以稍后再打开软件重试，或到 GitHub Releases 手动下载安装包。"
            ),
            Ask::Error,
        )
        .await;
        return Ok(());
    }

    // Windows 上安装程序启动后，更新插件会退出当前进程，并由安装包重新打开新版本。
    // 能走到这里说明当前平台没有自行退出，保持原窗口即可。
    Ok(())
}

enum Ask {
    Confirm,
    Error,
}

async fn ask(app: &AppHandle, title: &str, message: String, kind: Ask) -> bool {
    let app = app.clone();
    let title = title.to_string();
    tauri::async_runtime::spawn_blocking(move || {
        let mut dialog = app
            .dialog()
            .message(message)
            .title(title)
            .kind(match kind {
                Ask::Confirm => MessageDialogKind::Info,
                Ask::Error => MessageDialogKind::Error,
            })
            .buttons(match kind {
                Ask::Confirm => {
                    MessageDialogButtons::OkCancelCustom("立即更新".into(), "稍后".into())
                }
                Ask::Error => MessageDialogButtons::OkCustom("知道了".into()),
            });
        if let Some(window) = app.get_webview_window("main") {
            dialog = dialog.parent(&window);
        }
        dialog.blocking_show()
    })
    .await
    .unwrap_or(false)
}

fn shorten_notes(notes: &str) -> String {
    const MAX_CHARS: usize = 400;
    if notes.chars().count() <= MAX_CHARS {
        return notes.to_string();
    }
    let truncated: String = notes.chars().take(MAX_CHARS).collect();
    format!("{truncated}…")
}
