use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};
use tauri_plugin_opener::OpenerExt;

use crate::commands::AppState;

pub fn setup_tray(app: &AppHandle) -> tauri::Result<()> {
    let open_item = MenuItem::with_id(app, "open", "Open GraphSync", true, None::<&str>)?;
    let pause_item = MenuItem::with_id(app, "pause", "Pause Sync", true, None::<&str>)?;
    let resume_item = MenuItem::with_id(app, "resume", "Resume Sync", true, None::<&str>)?;
    let folder_item = MenuItem::with_id(app, "folder", "Open Sync Folder", true, None::<&str>)?;
    let settings_item = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
    let quit_item = PredefinedMenuItem::quit(app, Some("Quit"))?;
    let status_item = MenuItem::with_id(app, "status", "● Synced", false, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[
            &status_item,
            &open_item,
            &pause_item,
            &resume_item,
            &folder_item,
            &settings_item,
            &PredefinedMenuItem::separator(app)?,
            &quit_item,
        ],
    )?;

    let icon = app
        .default_window_icon()
        .cloned()
        .expect("default window icon");

    TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .tooltip("GraphSync")
        .on_menu_event(|app, event| {
            match event.id.as_ref() {
                "open" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "pause" => {
                    if let Some(state) = app.try_state::<AppState>() {
                        if let Some(engine) = state.sync.read().clone() {
                            let _ = engine.pause();
                        }
                    }
                }
                "resume" => {
                    if let Some(state) = app.try_state::<AppState>() {
                        if let Some(engine) = state.sync.read().clone() {
                            let _ = engine.resume();
                        }
                    }
                }
                "folder" => {
                    if let Some(state) = app.try_state::<AppState>() {
                        if let Some(folder) = state.config.read().sync_folder.clone() {
                            let _ = app.opener().open_path(folder, None::<&str>);
                        }
                    }
                }
                "settings" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                        let _ = window.eval("window.location.hash = '#/settings';");
                    }
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        })
        .build(app)?;

    Ok(())
}
