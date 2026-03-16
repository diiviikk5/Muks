//! VORTEX Core - Main shell orchestration

use std::time::{Duration, Instant};
use anyhow::Result;

/// Events that the shell can process
#[derive(Debug, Clone)]
pub enum VortexEvent {
    /// A frame needs to be rendered
    Render(RenderFrame),
    /// Time-based update tick
    Tick(Duration),
    /// Window management action
    WindowAction(WindowAction),
    /// System-level event from Windows
    SystemEvent(SystemEvent),
    /// Request to exit the shell
    Exit,
}

/// Frame data for rendering
#[derive(Debug, Clone)]
pub struct RenderFrame {
    pub delta_time: f32,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub screen_width: u32,
    pub screen_height: u32,
}

/// Window management actions
#[derive(Debug, Clone)]
pub enum WindowAction {
    Minimize,
    Maximize,
    Close,
    Move { x: i32, y: i32 },
    Resize { width: u32, height: u32 },
    SnapLeft,
    SnapRight,
    ShowMenu,
}

/// System events from Windows
#[derive(Debug, Clone)]
pub enum SystemEvent {
    FocusChanged { window_handle: isize },
    DisplayChanged,
    ThemeChanged,
    DpiChanged { scale: f32 },
    PowerEvent { event_type: PowerEventType },
}

/// Power-related events
#[derive(Debug, Clone)]
pub enum PowerEventType {
    Suspend,
    Resume,
    BatteryLow,
    PowerButton,
}

/// Core shell state
pub struct VortexCore {
    running: bool,
    start_time: Instant,
    frame_count: u64,
    fps: f64,
}

impl VortexCore {
    /// Create a new core instance
    pub fn new() -> Result<Self> {
        Ok(Self {
            running: true,
            start_time: Instant::now(),
            frame_count: 0,
            fps: 0.0,
        })
    }
    
    /// Check if shell is running
    pub fn is_running(&self) -> bool {
        self.running
    }
    
    /// Stop the shell
    pub fn stop(&mut self) {
        self.running = false;
    }
    
    /// Get uptime
    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }
    
    /// Get frame count
    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }
    
    /// Update frame stats
    pub fn update_frame(&mut self, delta: Duration) {
        self.frame_count += 1;
        if self.frame_count > 0 {
            let total = self.start_time.elapsed().as_secs_f64();
            if total > 0.0 {
                self.fps = self.frame_count as f64 / total;
            }
        }
    }
    
    /// Get current FPS
    pub fn fps(&self) -> f64 {
        self.fps
    }
}
