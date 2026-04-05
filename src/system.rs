use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::env;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfo {
    pub battery: Option<u8>,
    pub charging: bool,
    pub wifi_connected: Option<bool>,
    pub volume: Option<u8>,
    pub username: String,
    pub hostname: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppEntry {
    pub id: String,
    pub name: String,
    pub target: String,
    pub source: String,
}

pub fn get_system_info() -> SystemInfo {
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};

        let mut battery = None;
        let mut charging = false;

        unsafe {
            let mut power_status = SYSTEM_POWER_STATUS {
                ACLineStatus: 0,
                BatteryFlag: 0,
                BatteryLifePercent: 255,
                SystemStatusFlag: 0,
                BatteryLifeTime: 0,
                BatteryFullLifeTime: 0,
            };

            if GetSystemPowerStatus(&mut power_status) != 0 {
                if power_status.BatteryLifePercent != 255 {
                    battery = Some(power_status.BatteryLifePercent);
                }
                charging = power_status.ACLineStatus == 1;
            }
        }

        return SystemInfo {
            battery,
            charging,
            wifi_connected: None,
            volume: None,
            username: env::var("USERNAME").unwrap_or_else(|_| "User".to_string()),
            hostname: env::var("COMPUTERNAME").unwrap_or_else(|_| "Windows".to_string()),
        };
    }

    #[cfg(not(windows))]
    {
        SystemInfo {
            battery: None,
            charging: false,
            wifi_connected: None,
            volume: None,
            username: env::var("USER").unwrap_or_else(|_| "User".to_string()),
            hostname: env::var("HOSTNAME").unwrap_or_else(|_| "Machine".to_string()),
        }
    }
}

pub fn discover_apps() -> Vec<AppEntry> {
    let mut apps = known_apps();
    let mut seen_names = HashSet::new();

    for app in apps.values() {
        seen_names.insert(app.name.to_lowercase());
    }

    for root in app_search_roots() {
        if !root.exists() {
            continue;
        }

        for entry in WalkDir::new(root)
            .follow_links(false)
            .max_depth(5)
            .into_iter()
            .filter_map(Result::ok)
        {
            if !entry.file_type().is_file() {
                continue;
            }

            let path = entry.path();
            let Some(ext) = path.extension().and_then(|value| value.to_str()) else {
                continue;
            };

            if !matches!(ext.to_ascii_lowercase().as_str(), "lnk" | "url" | "exe") {
                continue;
            }

            let Some(stem) = path.file_stem().and_then(|value| value.to_str()) else {
                continue;
            };

            let name = normalize_name(stem);
            if name.len() < 2 || is_noise_app(&name) || !seen_names.insert(name.to_lowercase()) {
                continue;
            }

            let id = slugify(&name);
            apps.insert(
                id.clone(),
                AppEntry {
                    id,
                    name,
                    target: path.to_string_lossy().to_string(),
                    source: "start-menu".to_string(),
                },
            );
        }
    }

    let mut values: Vec<_> = apps.into_values().collect();
    values.sort_by(|left, right| left.name.cmp(&right.name));
    values.truncate(300);
    values
}

pub fn resolve_app_target(app_id: &str) -> Option<String> {
    known_apps().remove(app_id).map(|entry| entry.target)
}

pub fn launch_target(target: &str) -> Result<(), String> {
    #[cfg(windows)]
    {
        use std::process::Command;

        let resolved = if Path::new(target).exists() {
            target.to_string()
        } else {
            target.to_string()
        };

        Command::new("cmd")
            .args(["/C", "start", "", &resolved])
            .spawn()
            .map_err(|error| format!("Failed to open target `{}`: {}", target, error))?;

        return Ok(());
    }

    #[cfg(not(windows))]
    {
        let _ = target;
        Err("Launching apps is only supported on Windows right now".to_string())
    }
}

fn known_apps() -> HashMap<String, AppEntry> {
    [
        known_app("explorer", "Explorer", "explorer.exe", "system"),
        known_app("browser", "Browser", "https://www.google.com", "system"),
        known_app("terminal", "Terminal", "wt.exe", "system"),
        known_app("code", "VS Code", "code.exe", "system"),
        known_app("settings", "Settings", "ms-settings:", "system"),
        known_app("notepad", "Notepad", "notepad.exe", "system"),
    ]
    .into_iter()
    .map(|entry| (entry.id.clone(), entry))
    .collect()
}

fn known_app(id: &str, name: &str, target: &str, source: &str) -> AppEntry {
    AppEntry {
        id: id.to_string(),
        name: name.to_string(),
        target: target.to_string(),
        source: source.to_string(),
    }
}

fn app_search_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();

    for key in ["APPDATA", "ProgramData", "USERPROFILE", "PUBLIC"] {
        let Ok(base) = env::var(key) else {
            continue;
        };

        let base = PathBuf::from(base);
        match key {
            "APPDATA" | "ProgramData" => roots.push(base.join("Microsoft\\Windows\\Start Menu\\Programs")),
            "USERPROFILE" | "PUBLIC" => roots.push(base.join("Desktop")),
            _ => {}
        }
    }

    roots
}

fn normalize_name(value: &str) -> String {
    value
        .replace(" - Shortcut", "")
        .replace(" (1)", "")
        .trim()
        .to_string()
}

fn slugify(value: &str) -> String {
    let mut out = String::new();
    let mut last_dash = false;

    for ch in value.chars().flat_map(|ch| ch.to_lowercase()) {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            last_dash = false;
        } else if !last_dash {
            out.push('-');
            last_dash = true;
        }
    }

    out.trim_matches('-').to_string()
}

fn is_noise_app(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower.contains("uninstall")
        || lower.contains("readme")
        || lower.contains("help")
        || lower.contains("website")
        || lower.contains("license")
}
