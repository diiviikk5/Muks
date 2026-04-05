mod app_index;
mod rules;
mod windows_os;

use crate::config::{self, AppConfigV1};
use crate::system::{discover_apps, launch_target, resolve_app_target, AppEntry};
use anyhow::Result;
use parking_lot::RwLock;
use rules::{default_rules, AppRule};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellSnapshot {
    pub mode: String,
    pub healthy: bool,
    pub uptime_seconds: u64,
    pub startup_time_ms: u128,
    pub active_workspace_id: String,
    pub performance: PerformanceSnapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceSnapshot {
    pub app_index_size: usize,
    pub launch_count: u64,
    pub command_count: u64,
    pub index_refresh_count: u64,
    pub last_indexed_epoch_ms: Option<u64>,
    pub last_refresh_delta_apps: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceInfo {
    pub id: String,
    pub name: String,
    pub monitor_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowInfo {
    pub id: String,
    pub title: String,
    pub workspace_id: String,
    pub is_focused: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeInfo {
    pub id: String,
    pub name: String,
    pub is_active: bool,
    pub tokens: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginInfo {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub health: String,
    pub source: String,
    pub restart_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppIndexRefreshResult {
    pub previous_count: usize,
    pub new_count: usize,
    pub delta: i64,
    pub indexed_epoch_ms: u64,
}

pub struct AppRuntime {
    state: RwLock<RuntimeState>,
    config: RwLock<AppConfigV1>,
}

struct RuntimeState {
    mode: String,
    healthy: bool,
    started_at: Instant,
    startup_time_ms: u128,
    app_index: Vec<AppEntry>,
    last_indexed_epoch_ms: Option<u64>,
    launch_count: u64,
    command_count: u64,
    index_refresh_count: u64,
    last_refresh_delta_apps: i64,
    active_workspace_id: String,
    workspaces: Vec<WorkspaceInfo>,
    windows: Vec<WindowInfo>,
    window_workspace: HashMap<String, String>,
    focus_history: Vec<String>,
    rules: Vec<AppRule>,
    themes: Vec<ThemeInfo>,
    plugins: Vec<PluginInfo>,
}

impl AppRuntime {
    pub fn new() -> Result<Self> {
        let cfg = config::load_or_create_v1()?;
        let cached_apps = app_index::load_cache()?
            .map(|cache| cache.apps)
            .unwrap_or_default();

        let runtime = Self {
            state: RwLock::new(RuntimeState {
                mode: cfg.shell.default_mode.clone(),
                healthy: false,
                started_at: Instant::now(),
                startup_time_ms: 0,
                app_index: cached_apps,
                last_indexed_epoch_ms: None,
                launch_count: 0,
                command_count: 0,
                index_refresh_count: 0,
                last_refresh_delta_apps: 0,
                active_workspace_id: "ws-1".to_string(),
                workspaces: vec![
                    WorkspaceInfo {
                        id: "ws-1".to_string(),
                        name: "Primary".to_string(),
                        monitor_id: "monitor-1".to_string(),
                    },
                    WorkspaceInfo {
                        id: "ws-2".to_string(),
                        name: "Dev".to_string(),
                        monitor_id: "monitor-1".to_string(),
                    },
                    WorkspaceInfo {
                        id: "ws-3".to_string(),
                        name: "Comms".to_string(),
                        monitor_id: "monitor-1".to_string(),
                    },
                ],
                windows: Vec::new(),
                window_workspace: HashMap::new(),
                focus_history: Vec::new(),
                rules: default_rules(),
                themes: vec![
                    ThemeInfo {
                        id: "aura-flow".to_string(),
                        name: "Aura Flow".to_string(),
                        is_active: cfg.theme.current_theme == "aura-flow",
                        tokens: serde_json::json!({
                            "surface": "#12161f",
                            "accent": "#5b9bff",
                            "elevation": "glass"
                        }),
                    },
                    ThemeInfo {
                        id: "crystal-edge".to_string(),
                        name: "Crystal Edge".to_string(),
                        is_active: cfg.theme.current_theme == "crystal-edge",
                        tokens: serde_json::json!({
                            "surface": "#0f1320",
                            "accent": "#a8c9ff",
                            "elevation": "sharp-glass"
                        }),
                    },
                    ThemeInfo {
                        id: "midnight-pulse".to_string(),
                        name: "Midnight Pulse".to_string(),
                        is_active: cfg.theme.current_theme == "midnight-pulse",
                        tokens: serde_json::json!({
                            "surface": "#0a0c14",
                            "accent": "#7fd0ff",
                            "elevation": "neon-glass"
                        }),
                    },
                ],
                plugins: Vec::new(),
            }),
            config: RwLock::new(cfg),
        };

        Ok(runtime)
    }

    pub fn bootstrap(&self) {
        let _ = self.refresh_app_index();
        let _ = self.sync_windows_from_os();
        let mut state = self.state.write();
        state.healthy = true;
        state.startup_time_ms = state.started_at.elapsed().as_millis();
    }

    pub fn is_healthy(&self) -> bool {
        self.state.read().healthy
    }

    pub fn shell_get_state(&self) -> ShellSnapshot {
        self.bump_command_count();
        let state = self.state.read();
        ShellSnapshot {
            mode: state.mode.clone(),
            healthy: state.healthy,
            uptime_seconds: state.started_at.elapsed().as_secs(),
            startup_time_ms: state.startup_time_ms,
            active_workspace_id: state.active_workspace_id.clone(),
            performance: PerformanceSnapshot {
                app_index_size: state.app_index.len(),
                launch_count: state.launch_count,
                command_count: state.command_count,
                index_refresh_count: state.index_refresh_count,
                last_indexed_epoch_ms: state.last_indexed_epoch_ms,
                last_refresh_delta_apps: state.last_refresh_delta_apps,
            },
        }
    }

    pub fn shell_set_mode(&self, mode: &str) -> ShellSnapshot {
        self.bump_command_count();
        let normalized = match mode.trim().to_lowercase().as_str() {
            "safe" | "performance" | "normal" => mode.trim().to_lowercase(),
            _ => "normal".to_string(),
        };

        let mut state = self.state.write();
        state.mode = normalized;
        drop(state);
        self.shell_get_state()
    }

    pub fn refresh_app_index(&self) -> AppIndexRefreshResult {
        self.bump_command_count();
        let catalog = discover_apps();
        let previous_count;
        {
            let mut state = self.state.write();
            previous_count = state.app_index.len();
            state.app_index = catalog;
            state.last_indexed_epoch_ms = Some(current_epoch_ms());
            state.index_refresh_count += 1;
            state.last_refresh_delta_apps = state.app_index.len() as i64 - previous_count as i64;
        }

        let state = self.state.read();
        let _ = app_index::save_cache(&state.app_index);
        let indexed_epoch_ms = state.last_indexed_epoch_ms.unwrap_or_default();
        let new_count = state.app_index.len();
        let delta = new_count as i64 - previous_count as i64;

        AppIndexRefreshResult {
            previous_count,
            new_count,
            delta,
            indexed_epoch_ms,
        }
    }

    pub fn apps_search(&self, query: &str, limit: usize) -> Vec<AppEntry> {
        self.bump_command_count();
        let safe_limit = limit.clamp(1, 500);
        let state = self.state.read();

        if query.trim().is_empty() {
            return state.app_index.iter().take(safe_limit).cloned().collect();
        }

        let needle = query.to_lowercase();
        let mut ranked: Vec<(i32, AppEntry)> = state
            .app_index
            .iter()
            .filter_map(|app| {
                let name = app.name.to_lowercase();
                let source = app.source.to_lowercase();
                let score = if name == needle {
                    1000
                } else if name.starts_with(&needle) {
                    800
                } else if name.contains(&needle) {
                    620
                } else if source.contains(&needle) {
                    430
                } else {
                    return None;
                };
                Some((score - (name.len() as i32), app.clone()))
            })
            .collect();

        ranked.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.name.cmp(&b.1.name)));
        ranked
            .into_iter()
            .take(safe_limit)
            .map(|(_, app)| app)
            .collect()
    }

    pub fn apps_launch(&self, app_id: &str) -> Result<(), String> {
        self.bump_command_count();

        let target = resolve_app_target(app_id).or_else(|| {
            self.state
                .read()
                .app_index
                .iter()
                .find(|app| app.id == app_id)
                .map(|app| app.target.clone())
        });

        let launch_target_value = target.unwrap_or_else(|| app_id.to_string());
        launch_target(&launch_target_value)?;

        let mut state = self.state.write();
        state.launch_count += 1;

        if let Some(rule) = state
            .rules
            .iter()
            .find(|rule| rule.matches_process(app_id))
            .cloned()
        {
            state.active_workspace_id = rule.workspace_id;
        }

        Ok(())
    }

    pub fn windows_list(&self) -> Vec<WindowInfo> {
        self.bump_command_count();
        let _ = self.sync_windows_from_os();
        self.state.read().windows.clone()
    }

    pub fn windows_act(&self, window_id: &str, action: &str) -> Result<(), String> {
        self.bump_command_count();
        windows_os::perform_action(window_id, action)?;
        let _ = self.sync_windows_from_os();
        Ok(())
    }

    pub fn windows_move_to_workspace(
        &self,
        window_id: &str,
        workspace_id: &str,
    ) -> Result<(), String> {
        self.bump_command_count();
        let mut state = self.state.write();

        if !state.workspaces.iter().any(|workspace| workspace.id == workspace_id) {
            return Err(format!("workspace `{}` does not exist", workspace_id));
        }

        let Some(window) = state.windows.iter_mut().find(|window| window.id == window_id) else {
            return Err(format!("window `{}` not found", window_id));
        };

        window.workspace_id = workspace_id.to_string();
        state
            .window_workspace
            .insert(window_id.to_string(), workspace_id.to_string());
        Ok(())
    }

    pub fn workspaces_list(&self) -> Vec<WorkspaceInfo> {
        self.bump_command_count();
        self.state.read().workspaces.clone()
    }

    pub fn workspaces_create(&self, name: &str) -> WorkspaceInfo {
        self.bump_command_count();
        let mut state = self.state.write();
        let id = format!("ws-{}", state.workspaces.len() + 1);
        let workspace = WorkspaceInfo {
            id,
            name: name.trim().to_string(),
            monitor_id: "monitor-1".to_string(),
        };
        state.workspaces.push(workspace.clone());
        workspace
    }

    pub fn workspaces_switch(&self, workspace_id: &str) -> Result<WorkspaceInfo, String> {
        self.bump_command_count();
        let mut state = self.state.write();

        let Some(workspace) = state
            .workspaces
            .iter()
            .find(|workspace| workspace.id == workspace_id)
            .cloned()
        else {
            return Err(format!("workspace `{}` does not exist", workspace_id));
        };

        state.active_workspace_id = workspace_id.to_string();
        Ok(workspace)
    }

    pub fn themes_list(&self) -> Vec<ThemeInfo> {
        self.bump_command_count();
        self.state.read().themes.clone()
    }

    pub fn themes_apply(&self, theme_id: &str) -> Result<ThemeInfo, String> {
        self.bump_command_count();
        let mut state = self.state.write();

        if !state.themes.iter().any(|theme| theme.id == theme_id) {
            return Err(format!("theme `{}` not found", theme_id));
        }

        let mut active = None;
        for theme in state.themes.iter_mut() {
            theme.is_active = theme.id == theme_id;
            if theme.is_active {
                active = Some(theme.clone());
            }
        }

        drop(state);

        {
            let mut cfg = self.config.write();
            cfg.theme.current_theme = theme_id.to_string();
            if let Err(error) = config::save_v1(&cfg) {
                tracing::warn!("failed to persist theme selection: {}", error);
            }
        }

        active.ok_or_else(|| "active theme missing after apply".to_string())
    }

    pub fn themes_preview(&self, theme_json: &str) -> Result<ThemeInfo, String> {
        self.bump_command_count();
        let tokens: serde_json::Value = serde_json::from_str(theme_json)
            .map_err(|error| format!("invalid theme JSON: {}", error))?;

        Ok(ThemeInfo {
            id: "preview".to_string(),
            name: "Preview Theme".to_string(),
            is_active: false,
            tokens,
        })
    }

    pub fn themes_save(&self, theme_json: &str) -> Result<ThemeInfo, String> {
        self.bump_command_count();
        let mut theme = self.themes_preview(theme_json)?;
        let mut state = self.state.write();
        let id = format!("custom-{}", state.themes.len() + 1);
        theme.id = id;
        theme.name = format!("Custom Theme {}", state.themes.len() + 1);
        state.themes.push(theme.clone());
        Ok(theme)
    }

    pub fn plugins_list(&self) -> Vec<PluginInfo> {
        self.bump_command_count();
        self.state.read().plugins.clone()
    }

    pub fn plugins_enable(&self, plugin_id: &str) -> Result<PluginInfo, String> {
        self.bump_command_count();
        self.set_plugin_state(plugin_id, true)
    }

    pub fn plugins_disable(&self, plugin_id: &str) -> Result<PluginInfo, String> {
        self.bump_command_count();
        self.set_plugin_state(plugin_id, false)
    }

    pub fn plugins_install(&self, source: &str) -> Result<PluginInfo, String> {
        self.bump_command_count();
        let mut state = self.state.write();
        let id = format!("plugin-{}", slugify(source));

        if let Some(existing) = state.plugins.iter().find(|plugin| plugin.id == id).cloned() {
            return Ok(existing);
        }

        let plugin = PluginInfo {
            id,
            name: format!("Plugin {}", state.plugins.len() + 1),
            enabled: false,
            health: "healthy".to_string(),
            source: source.to_string(),
            restart_count: 0,
        };

        state.plugins.push(plugin.clone());
        Ok(plugin)
    }

    fn set_plugin_state(&self, plugin_id: &str, enabled: bool) -> Result<PluginInfo, String> {
        let mut state = self.state.write();
        let Some(plugin) = state.plugins.iter_mut().find(|plugin| plugin.id == plugin_id) else {
            return Err(format!("plugin `{}` not found", plugin_id));
        };

        plugin.enabled = enabled;
        plugin.health = if enabled { "healthy" } else { "disabled" }.to_string();
        Ok(plugin.clone())
    }

    pub fn sync_windows_from_os(&self) -> usize {
        let samples = windows_os::capture_windows();
        let mut state = self.state.write();
        let active_workspace = state.active_workspace_id.clone();

        state.windows = samples
            .iter()
            .map(|sample| {
                let workspace_id = state
                    .window_workspace
                    .get(&sample.id)
                    .cloned()
                    .unwrap_or_else(|| active_workspace.clone());

                WindowInfo {
                    id: sample.id.clone(),
                    title: sample.title.clone(),
                    workspace_id,
                    is_focused: sample.is_focused,
                }
            })
            .collect();

        if let Some(focused_id) = state
            .windows
            .iter()
            .find(|window| window.is_focused)
            .map(|window| window.id.clone())
        {
            if state
                .focus_history
                .last()
                .map(|id| id != &focused_id)
                .unwrap_or(true)
            {
                state.focus_history.push(focused_id);
                if state.focus_history.len() > 100 {
                    let _ = state.focus_history.remove(0);
                }
            }
        }

        state.windows.len()
    }

    fn bump_command_count(&self) {
        self.state.write().command_count += 1;
    }
}

fn current_epoch_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|value| value.as_millis() as u64)
        .unwrap_or_default()
}

fn slugify(input: &str) -> String {
    let mut output = String::new();
    let mut last_dash = false;

    for ch in input.chars().flat_map(|ch| ch.to_lowercase()) {
        if ch.is_ascii_alphanumeric() {
            output.push(ch);
            last_dash = false;
        } else if !last_dash {
            output.push('-');
            last_dash = true;
        }
    }

    output.trim_matches('-').to_string()
}
