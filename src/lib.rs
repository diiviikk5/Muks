//! VORTEX Shell - Tauri Backend
//! A living, intelligent desktop shell for Windows

mod core;
mod platform;
mod shell;
mod biomes;
mod ai;
mod config;

use core::VortexCore;
use tauri::Manager;

pub fn run() {
    // Initialize logging
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(true)
        .with_line_number(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    tracing::info!("═══════════════════════════════════════════════════════════════");
    tracing::info!("  VORTEX Shell v0.1.0 - Living Desktop Experience");
    tracing::info!("═══════════════════════════════════════════════════════════════");

    tauri::Builder::default()
        .setup(|app| {
            tracing::info!("Setting up VORTEX Shell...");

            // Initialize core system
            let core = VortexCore::new()?;
            app.manage(core);
            tracing::info!("Core system ready");

            // Get the main window
            let window = app.get_webview_window("main").unwrap();

            // Set up position (bottom of screen, taskbar height)
            #[cfg(windows)]
            {
                use windows_sys::Win32::UI::WindowsAndMessaging::GetSystemMetrics;
                use windows_sys::Win32::UI::WindowsAndMessaging::SM_CXSCREEN;
                use windows_sys::Win32::UI::WindowsAndMessaging::SM_CYSCREEN;

                unsafe {
                    let screen_width = GetSystemMetrics(SM_CXSCREEN) as i32;
                    let screen_height = GetSystemMetrics(SM_CYSCREEN) as i32;
                    let taskbar_height = 48;

                    // Position at bottom of screen
                    window.set_position(tauri::Position::Physical(
                        tauri::PhysicalPosition { x: 0, y: screen_height - taskbar_height }
                    )).ok();

                    // Set size to full width, taskbar height
                    window.set_size(tauri::Size::Physical(
                        tauri::PhysicalSize { width: screen_width as u32, height: taskbar_height as u32 }
                    )).ok();
                }
            }

            tracing::info!("Window positioned");
            tracing::info!("═══════════════════════════════════════════════════════════════");
            tracing::info!("  VORTEX Shell initialized successfully!");
            tracing::info!("═══════════════════════════════════════════════════════════════");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_core_info,
            get_biome_name,
            set_biome,
            get_screen_size,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Get core system information
#[tauri::command]
fn get_core_info(core: tauri::State<'_, VortexCore>) -> serde_json::Value {
    serde_json::json!({
        "running": core.is_running(),
        "frame_count": core.frame_count(),
        "fps": core.fps(),
        "uptime_seconds": core.uptime().as_secs()
    })
}

/// Get current biome name
#[tauri::command]
fn get_biome_name() -> String {
    "AuraFlow".to_string()
}

/// Set the current biome
#[tauri::command]
fn set_biome(biome: String) -> Result<(), String> {
    tracing::info!("Switching biome to: {}", biome);
    // Biome switching would be handled here
    Ok(())
}

/// Get screen dimensions
#[tauri::command]
fn get_screen_size() -> (u32, u32) {
    #[cfg(windows)]
    {
        unsafe {
            use windows_sys::Win32::UI::WindowsAndMessaging::GetSystemMetrics;
            use windows_sys::Win32::UI::WindowsAndMessaging::SM_CXSCREEN;
            use windows_sys::Win32::UI::WindowsAndMessaging::SM_CYSCREEN;
            return (GetSystemMetrics(SM_CXSCREEN) as u32, GetSystemMetrics(SM_CYSCREEN) as u32);
        }
    }
    #[cfg(not(windows))]
    (1920, 1080)
}
