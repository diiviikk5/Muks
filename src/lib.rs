//! VORTEX Shell - Tauri Backend
//! A living, intelligent desktop shell for Windows

mod ai;
mod biomes;
mod config;
mod core;
mod platform;
mod runtime;
mod shell;
mod system;

use core::VortexCore;
use runtime::{
    AppIndexRefreshResult, AppRuntime, PluginInfo, ShellSnapshot, ThemeInfo, WindowInfo,
    WorkspaceInfo,
};
use serde::Serialize;
use system::get_system_info as read_system_info;
use tauri::Emitter;
use tauri::Manager;

const TASKBAR_HEIGHT: i32 = 112;
const PANEL_HEIGHT: i32 = 860;

const TOPIC_SYSTEM_STATE_CHANGED: &str = "system.state.changed";
const TOPIC_WINDOWS_CHANGED: &str = "windows.changed";
const TOPIC_WORKSPACE_CHANGED: &str = "workspace.changed";
const TOPIC_APPS_INDEX_UPDATED: &str = "apps.index.updated";
const TOPIC_THEME_CHANGED: &str = "theme.changed";
const TOPIC_PLUGIN_HEALTH: &str = "plugin.health";

pub fn run() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(true)
        .with_line_number(true)
        .finish();

    let _ = tracing::subscriber::set_global_default(subscriber);

    tracing::info!("===============================================================");
    tracing::info!("VORTEX Shell v0.1.0");
    tracing::info!("===============================================================");

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_log::Builder::default().build())
        .setup(|app| {
            tracing::info!("Setting up VORTEX Shell...");

            let core = VortexCore::new()?;
            app.manage(core);

            let runtime = AppRuntime::new()?;
            runtime.bootstrap();
            app.manage(runtime);

            if let Some(window) = app.get_webview_window("main") {
                position_window(&window, TASKBAR_HEIGHT);
            }

            #[cfg(windows)]
            {
                let runtime = app.state::<AppRuntime>();
                if runtime.is_healthy() {
                    set_windows_taskbar_visibility(false);
                }
            }

            tracing::info!("VORTEX Shell initialized");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            shell_get_state,
            shell_subscribe,
            shell_set_mode,
            apps_search,
            apps_refresh_index,
            apps_launch,
            windows_list,
            windows_act,
            windows_move_to_workspace,
            workspaces_list,
            workspaces_create,
            workspaces_switch,
            themes_list,
            themes_apply,
            themes_preview,
            themes_save,
            plugins_list,
            plugins_enable,
            plugins_disable,
            plugins_install,
            get_core_info,
            get_biome_name,
            set_biome,
            get_screen_size,
            get_system_info,
            get_app_catalog,
            minimize_window,
            maximize_window,
            set_menu_open,
            close_window,
            open_app,
            show_desktop,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|_app_handle, event| {
        #[cfg(windows)]
        if matches!(
            event,
            tauri::RunEvent::ExitRequested { .. } | tauri::RunEvent::Exit
        ) {
            set_windows_taskbar_visibility(true);
        }
    });
}

#[cfg(windows)]
fn set_windows_taskbar_visibility(show: bool) {
    use windows_sys::Win32::UI::WindowsAndMessaging::{FindWindowA, ShowWindow, SW_HIDE, SW_SHOW};

    let mode = if show { SW_SHOW } else { SW_HIDE };

    unsafe {
        let taskbar = FindWindowA(b"Shell_TrayWnd\0".as_ptr(), std::ptr::null());
        if !taskbar.is_null() {
            ShowWindow(taskbar, mode);
        }

        let secondary = FindWindowA(b"Shell_SecondaryTrayWnd\0".as_ptr(), std::ptr::null());
        if !secondary.is_null() {
            ShowWindow(secondary, mode);
        }
    }
}

fn position_window(window: &tauri::WebviewWindow, height: i32) {
    #[cfg(windows)]
    {
        use windows_sys::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

        unsafe {
            let screen_width = GetSystemMetrics(SM_CXSCREEN);
            let screen_height = GetSystemMetrics(SM_CYSCREEN);

            let _ = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                x: 0,
                y: screen_height - height,
            }));

            let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                width: screen_width as u32,
                height: height as u32,
            }));
        }
    }
}

fn emit_topic<T: Serialize>(app: &tauri::AppHandle, topic: &str, payload: &T) {
    if let Err(error) = app.emit(topic, payload) {
        tracing::warn!("failed to emit topic {}: {}", topic, error);
    }
}

#[tauri::command]
fn shell_get_state(runtime: tauri::State<'_, AppRuntime>) -> ShellSnapshot {
    runtime.shell_get_state()
}

#[tauri::command]
fn shell_subscribe(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, AppRuntime>,
) -> ShellSnapshot {
    let snapshot = runtime.shell_get_state();
    emit_topic(&app, TOPIC_SYSTEM_STATE_CHANGED, &snapshot);
    snapshot
}

#[tauri::command]
fn shell_set_mode(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, AppRuntime>,
    mode: String,
) -> ShellSnapshot {
    let snapshot = runtime.shell_set_mode(&mode);
    emit_topic(&app, TOPIC_SYSTEM_STATE_CHANGED, &snapshot);
    snapshot
}

#[tauri::command]
fn apps_search(
    runtime: tauri::State<'_, AppRuntime>,
    query: String,
    limit: Option<usize>,
) -> Vec<system::AppEntry> {
    runtime.apps_search(&query, limit.unwrap_or(50))
}

#[tauri::command]
fn apps_refresh_index(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, AppRuntime>,
) -> AppIndexRefreshResult {
    let stats = runtime.refresh_app_index();
    emit_topic(&app, TOPIC_APPS_INDEX_UPDATED, &stats);
    emit_topic(&app, TOPIC_SYSTEM_STATE_CHANGED, &runtime.shell_get_state());
    stats
}

#[tauri::command]
fn apps_launch(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, AppRuntime>,
    app_id: String,
) -> Result<(), String> {
    runtime.apps_launch(&app_id)?;
    let snapshot = runtime.shell_get_state();
    emit_topic(&app, TOPIC_SYSTEM_STATE_CHANGED, &snapshot);
    Ok(())
}

#[tauri::command]
fn windows_list(runtime: tauri::State<'_, AppRuntime>) -> Vec<WindowInfo> {
    runtime.windows_list()
}

#[tauri::command]
fn windows_act(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, AppRuntime>,
    window_id: String,
    action: String,
) -> Result<(), String> {
    runtime.windows_act(&window_id, &action)?;
    emit_topic(&app, TOPIC_WINDOWS_CHANGED, &runtime.windows_list());
    Ok(())
}

#[tauri::command]
fn windows_move_to_workspace(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, AppRuntime>,
    window_id: String,
    workspace_id: String,
) -> Result<(), String> {
    runtime.windows_move_to_workspace(&window_id, &workspace_id)?;
    emit_topic(&app, TOPIC_WINDOWS_CHANGED, &runtime.windows_list());
    Ok(())
}

#[tauri::command]
fn workspaces_list(runtime: tauri::State<'_, AppRuntime>) -> Vec<WorkspaceInfo> {
    runtime.workspaces_list()
}

#[tauri::command]
fn workspaces_create(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, AppRuntime>,
    name: String,
) -> WorkspaceInfo {
    let workspace = runtime.workspaces_create(&name);
    emit_topic(&app, TOPIC_WORKSPACE_CHANGED, &runtime.workspaces_list());
    workspace
}

#[tauri::command]
fn workspaces_switch(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, AppRuntime>,
    workspace_id: String,
) -> Result<WorkspaceInfo, String> {
    let workspace = runtime.workspaces_switch(&workspace_id)?;
    emit_topic(&app, TOPIC_WORKSPACE_CHANGED, &workspace);
    Ok(workspace)
}

#[tauri::command]
fn themes_list(runtime: tauri::State<'_, AppRuntime>) -> Vec<ThemeInfo> {
    runtime.themes_list()
}

#[tauri::command]
fn themes_apply(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, AppRuntime>,
    theme_id: String,
) -> Result<ThemeInfo, String> {
    let theme = runtime.themes_apply(&theme_id)?;
    emit_topic(&app, TOPIC_THEME_CHANGED, &theme);
    Ok(theme)
}

#[tauri::command]
fn themes_preview(runtime: tauri::State<'_, AppRuntime>, theme_json: String) -> Result<ThemeInfo, String> {
    runtime.themes_preview(&theme_json)
}

#[tauri::command]
fn themes_save(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, AppRuntime>,
    theme_json: String,
) -> Result<ThemeInfo, String> {
    let theme = runtime.themes_save(&theme_json)?;
    emit_topic(&app, TOPIC_THEME_CHANGED, &theme);
    Ok(theme)
}

#[tauri::command]
fn plugins_list(runtime: tauri::State<'_, AppRuntime>) -> Vec<PluginInfo> {
    runtime.plugins_list()
}

#[tauri::command]
fn plugins_enable(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, AppRuntime>,
    plugin_id: String,
) -> Result<PluginInfo, String> {
    let plugin = runtime.plugins_enable(&plugin_id)?;
    emit_topic(&app, TOPIC_PLUGIN_HEALTH, &plugin);
    Ok(plugin)
}

#[tauri::command]
fn plugins_disable(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, AppRuntime>,
    plugin_id: String,
) -> Result<PluginInfo, String> {
    let plugin = runtime.plugins_disable(&plugin_id)?;
    emit_topic(&app, TOPIC_PLUGIN_HEALTH, &plugin);
    Ok(plugin)
}

#[tauri::command]
fn plugins_install(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, AppRuntime>,
    path_or_url: String,
) -> Result<PluginInfo, String> {
    let plugin = runtime.plugins_install(&path_or_url)?;
    emit_topic(&app, TOPIC_PLUGIN_HEALTH, &plugin);
    Ok(plugin)
}

#[tauri::command]
fn get_core_info(core: tauri::State<'_, VortexCore>) -> serde_json::Value {
    serde_json::json!({
        "running": core.is_running(),
        "frame_count": core.frame_count(),
        "fps": core.fps(),
        "uptime_seconds": core.uptime().as_secs()
    })
}

#[tauri::command]
fn get_biome_name() -> String {
    "AuraFlow".to_string()
}

#[tauri::command]
fn set_biome(biome: String) -> Result<(), String> {
    tracing::info!("Switching biome to: {}", biome);
    Ok(())
}

#[tauri::command]
fn get_screen_size() -> (u32, u32) {
    #[cfg(windows)]
    {
        unsafe {
            use windows_sys::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
            return (
                GetSystemMetrics(SM_CXSCREEN) as u32,
                GetSystemMetrics(SM_CYSCREEN) as u32,
            );
        }
    }

    #[cfg(not(windows))]
    {
        (1920, 1080)
    }
}

#[tauri::command]
fn get_system_info() -> system::SystemInfo {
    read_system_info()
}

#[tauri::command]
fn get_app_catalog(runtime: tauri::State<'_, AppRuntime>) -> Result<Vec<system::AppEntry>, String> {
    Ok(runtime.apps_search("", 300))
}

#[tauri::command]
async fn minimize_window(window: tauri::Window) -> Result<(), String> {
    window.minimize().map_err(|error| error.to_string())
}

#[tauri::command]
async fn maximize_window(window: tauri::Window) -> Result<(), String> {
    if window.is_maximized().unwrap_or(false) {
        window.unmaximize().map_err(|error| error.to_string())
    } else {
        window.maximize().map_err(|error| error.to_string())
    }
}

#[tauri::command]
fn set_menu_open(window: tauri::Window, is_open: bool) {
    let height = if is_open { PANEL_HEIGHT } else { TASKBAR_HEIGHT };

    #[cfg(windows)]
    {
        use windows_sys::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

        unsafe {
            let screen_width = GetSystemMetrics(SM_CXSCREEN) as u32;
            let screen_height = GetSystemMetrics(SM_CYSCREEN);

            let _ = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                x: 0,
                y: screen_height - height,
            }));

            let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                width: screen_width,
                height: height as u32,
            }));
        }
    }
}

#[tauri::command]
async fn close_window(app: tauri::AppHandle) -> Result<(), String> {
    #[cfg(windows)]
    set_windows_taskbar_visibility(true);

    app.exit(0);
    Ok(())
}

#[tauri::command]
async fn open_app(runtime: tauri::State<'_, AppRuntime>, app_id: String) -> Result<(), String> {
    runtime.apps_launch(&app_id)
}

#[tauri::command]
async fn show_desktop() -> Result<(), String> {
    tracing::info!("Show desktop requested");
    Ok(())
}
