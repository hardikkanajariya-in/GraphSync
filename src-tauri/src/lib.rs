mod commands;
mod config;
mod device;
mod filesystem;
mod hashing;
mod network;
mod sync;
mod tray;

use commands::AppState;
use tauri::Manager;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn init_logging() {
    let log_dir = config::log_dir().ok();
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,graphsync_lib=debug,webrtc=warn"));

    if let Some(dir) = log_dir {
        let file_appender = tracing_appender::rolling::daily(dir, "graphsync.log");
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
            .init();
        std::mem::forget(_guard);
    } else {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer())
            .init();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_logging();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let state = AppState::new().expect("failed to initialize app state");
            app.manage(state);

            tray::setup_tray(app.handle())?;

            let app_handle = app.handle().clone();
            let state = app.state::<AppState>();
            if state.config.read().setup_complete {
                let config = state.config.clone();
                let engine = sync::SyncEngine::new(app_handle.clone(), config);
                let handle = sync::spawn_engine(engine.clone());
                state.sync.write().replace(engine);
                state.engine.write().replace(handle);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::get_status,
            commands::save_setup,
            commands::start_sync,
            commands::pause_sync,
            commands::resume_sync,
            commands::update_settings,
            commands::reset_device,
            commands::get_log_path,
            commands::show_main_window,
            commands::hide_main_window,
            commands::is_setup_complete,
        ])
        .run(tauri::generate_context!())
        .expect("error while running GraphSync");
}
