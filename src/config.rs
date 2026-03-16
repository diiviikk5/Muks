//! VORTEX Configuration

use serde::{Deserialize, Serialize};

/// VORTEX Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Shell settings
    pub shell: ShellConfig,
    /// Visual settings
    pub visual: VisualConfig,
    /// AI settings
    pub ai: AIConfig,
    /// Performance settings
    pub performance: PerformanceConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            shell: ShellConfig::default(),
            visual: VisualConfig::default(),
            ai: AIConfig::default(),
            performance: PerformanceConfig::default(),
        }
    }
}

/// Shell-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellConfig {
    /// Auto-start with Windows
    pub auto_start: bool,
    /// Show system tray icon
    pub show_tray: bool,
    /// Replace explorer taskbar
    pub replace_taskbar: bool,
    /// Default biome on startup
    pub default_biome: String,
}

impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            auto_start: false,
            show_tray: true,
            replace_taskbar: false,
            default_biome: "AuraFlow".to_string(),
        }
    }
}

/// Visual configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualConfig {
    /// Enable animations
    pub animations: bool,
    /// Animation speed (0.5 - 2.0)
    pub animation_speed: f32,
    /// Enable particles
    pub particles: bool,
    /// Enable blur effects
    pub blur: bool,
    /// Custom accent color
    pub accent_color: Option<String>,
}

impl Default for VisualConfig {
    fn default() -> Self {
        Self {
            animations: true,
            animation_speed: 1.0,
            particles: true,
            blur: true,
            accent_color: None,
        }
    }
}

/// AI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    /// Enable AI features
    pub enabled: bool,
    /// Enable context agent
    pub context_agent: bool,
    /// Enable predictive workspace
    pub predictive_workspace: bool,
    /// Enable summarizer
    pub summarizer: bool,
    /// Model path (for custom models)
    pub model_path: Option<String>,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            context_agent: false,
            predictive_workspace: false,
            summarizer: false,
            model_path: None,
        }
    }
}

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Target FPS
    pub target_fps: u32,
    /// Enable vsync
    pub vsync: bool,
    /// Hardware acceleration
    pub hardware_acceleration: bool,
    /// Maximum memory (MB)
    pub max_memory_mb: u32,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            target_fps: 60,
            vsync: true,
            hardware_acceleration: true,
            max_memory_mb: 512,
        }
    }
}
