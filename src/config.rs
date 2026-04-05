//! VORTEX Configuration v1

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const CONFIG_V1_PATH: &str = "config/v1.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfigV1 {
    pub shell: ShellConfig,
    pub workspaces: WorkspaceConfig,
    pub theme: ThemeConfig,
    pub widgets: WidgetConfig,
    pub plugins: PluginConfig,
    pub performance: PerformanceConfig,
    pub accessibility: AccessibilityConfig,
}

impl Default for AppConfigV1 {
    fn default() -> Self {
        Self {
            shell: ShellConfig::default(),
            workspaces: WorkspaceConfig::default(),
            theme: ThemeConfig::default(),
            widgets: WidgetConfig::default(),
            plugins: PluginConfig::default(),
            performance: PerformanceConfig::default(),
            accessibility: AccessibilityConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellConfig {
    pub auto_start: bool,
    pub replace_explorer_opt_in: bool,
    pub start_in_safe_mode: bool,
    pub default_mode: String,
}

impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            auto_start: false,
            replace_explorer_opt_in: false,
            start_in_safe_mode: false,
            default_mode: "normal".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub default_count: u8,
    pub per_monitor: bool,
    pub remember_last_active: bool,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            default_count: 3,
            per_monitor: true,
            remember_last_active: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub current_theme: String,
    pub animations: bool,
    pub gpu_effects: bool,
    pub blur: bool,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            current_theme: "aura-flow".to_string(),
            animations: true,
            gpu_effects: true,
            blur: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetConfig {
    pub enabled: bool,
    pub sandboxed_js: bool,
    pub max_widgets: u16,
}

impl Default for WidgetConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            sandboxed_js: true,
            max_widgets: 24,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub enabled: bool,
    pub curated_beta_only: bool,
    pub max_restart_attempts: u8,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            curated_beta_only: true,
            max_restart_attempts: 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub target_fps: u16,
    pub launcher_budget_ms_p95: u16,
    pub search_budget_ms_p95: u16,
    pub idle_memory_budget_mb: u16,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            target_fps: 60,
            launcher_budget_ms_p95: 250,
            search_budget_ms_p95: 40,
            idle_memory_budget_mb: 120,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityConfig {
    pub high_contrast: bool,
    pub keyboard_only_mode: bool,
    pub reduced_motion: bool,
    pub scale_factor: f32,
}

impl Default for AccessibilityConfig {
    fn default() -> Self {
        Self {
            high_contrast: false,
            keyboard_only_mode: false,
            reduced_motion: false,
            scale_factor: 1.0,
        }
    }
}

pub fn config_v1_path() -> PathBuf {
    Path::new(CONFIG_V1_PATH).to_path_buf()
}

pub fn load_or_create_v1() -> Result<AppConfigV1> {
    let path = config_v1_path();

    if path.exists() {
        let raw = fs::read_to_string(&path)
            .with_context(|| format!("failed to read config file at {}", path.display()))?;
        let parsed: AppConfigV1 =
            toml::from_str(&raw).context("failed to parse config/v1.toml")?;
        return Ok(parsed);
    }

    let default = AppConfigV1::default();
    save_v1(&default)?;
    Ok(default)
}

pub fn save_v1(config: &AppConfigV1) -> Result<()> {
    let path = config_v1_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!("failed to create config directory {}", parent.display())
        })?;
    }

    let encoded = toml::to_string_pretty(config).context("failed to serialize config v1")?;
    fs::write(&path, encoded)
        .with_context(|| format!("failed to write config file at {}", path.display()))?;

    Ok(())
}
