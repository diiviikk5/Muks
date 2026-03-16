//! VORTEX Platform - Windows GUI Shell
//! 
//! Platform module - handles Windows integration with actual GUI shell

use crate::core::SystemEvent;
use crate::biomes::BiomeType;
use anyhow::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

// Win32 imports
#[cfg(windows)]
use windows_sys::Win32::Foundation::*;
#[cfg(windows)]
use windows_sys::Win32::UI::WindowsAndMessaging::*;
#[cfg(windows)]
use windows_sys::Win32::Graphics::Gdi::*;

/// VORTEX Shell state - shared with window
static mut VORTEX_SHELL: Option<*mut VortexShellState> = None;

/// Shared shell state
struct VortexShellState {
    running: bool,
    biome: BiomeType,
    start_time: Instant,
    frame_count: u64,
}

/// Helper to create RGB color
fn rgb(r: u8, g: u8, b: u8) -> COLORREF {
    ((b as u32) << 16) | ((g as u32) << 8) | (r as u32)
}

/// Windows Platform with real GUI
pub struct WindowsPlatform {
    pub hwnd: *mut std::ffi::c_void,
    should_exit: AtomicBool,
    running: AtomicBool,
    biome: BiomeType,
    screen_width: i32,
    screen_height: i32,
}

impl WindowsPlatform {
    pub fn new() -> Result<Self> {
        let (w, h) = Self::get_screen_dimensions();
        
        Ok(Self {
            hwnd: std::ptr::null_mut(),
            should_exit: AtomicBool::new(false),
            running: AtomicBool::new(false),
            biome: BiomeType::AuraFlow,
            screen_width: w,
            screen_height: h,
        })
    }
    
    fn get_screen_dimensions() -> (i32, i32) {
        #[cfg(windows)]
        {
            unsafe {
                let w = GetSystemMetrics(SM_CXSCREEN);
                let h = GetSystemMetrics(SM_CYSCREEN);
                return (w as i32, h as i32);
            }
        }
        #[cfg(not(windows))]
        (1920, 1080)
    }
    
    /// Create the main VORTEX shell window
    pub fn create_window(&mut self) -> Result<()> {
        tracing::info!("Creating VORTEX shell window...");
        
        #[cfg(windows)]
        {
            unsafe {
                // Register window class
                let class_name: Vec<u16> = "VortexShellClass\0".encode_utf16().collect();
                
                let wc = WNDCLASSEXW {
                    cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                    style: CS_HREDRAW | CS_VREDRAW,
                    lpfnWndProc: Some(shell_wnd_proc),
                    cbClsExtra: 0,
                    cbWndExtra: 0,
                    hInstance: std::ptr::null_mut(),
                    hIcon: std::ptr::null_mut(),
                    hCursor: LoadCursorW(std::ptr::null_mut(), IDI_APPLICATION),
                    hbrBackground: CreateSolidBrush(rgb(15, 20, 30)),
                    lpszMenuName: std::ptr::null(),
                    lpszClassName: class_name.as_ptr(),
                    hIconSm: std::ptr::null_mut(),
                };
                
                RegisterClassExW(&wc);
                
                // Create window - positioned at bottom of screen (taskbar)
                let window_name: Vec<u16> = "VORTEX Shell\0".encode_utf16().collect();
                
                self.hwnd = CreateWindowExW(
                    WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_LAYERED,
                    class_name.as_ptr(),
                    window_name.as_ptr(),
                    WS_POPUP | WS_VISIBLE,
                    0,
                    self.screen_height - 48,
                    self.screen_width,
                    48,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut()
                );
                
                if self.hwnd.is_null() {
                    return Err(anyhow::anyhow!("Failed to create window"));
                }
                
                // Set layered window for transparency
                let bg_color = rgb(15, 20, 30);
                SetLayeredWindowAttributes(self.hwnd, bg_color, 255, LWA_COLORKEY);
                
                // Show window
                ShowWindow(self.hwnd, SW_SHOW);
                UpdateWindow(self.hwnd);
                
                // Initialize shared state
                let state = Box::into_raw(Box::new(VortexShellState {
                    running: true,
                    biome: BiomeType::AuraFlow,
                    start_time: Instant::now(),
                    frame_count: 0,
                }));
                VORTEX_SHELL = Some(state);
                
                tracing::info!("VORTEX shell window created");
            }
        }
        
        #[cfg(not(windows))]
        {
            self.hwnd = 1 as *mut _;
        }
        
        Ok(())
    }
    
    /// Run the shell message loop
    pub fn run_message_loop(&mut self) {
        self.running.store(true, Ordering::SeqCst);
        
        println!("");
        println!("╔═══════════════════════════════════════════════════════════════════════════════╗");
        println!("║                    V O R T E X   S H E L L                             ║");
        println!("║                 Living Desktop Experience                                ║");
        println!("╠═══════════════════════════════════════════════════════════════════════════════╣");
        println!("║  Window: {}x48 | Running at 60 FPS                                   ║", self.screen_width);
        println!("║  Close the window or press Ctrl+C to quit                             ║");
        println!("╚═══════════════════════════════════════════════════════════════════════════════╝");
        
        tracing::info!("VORTEX shell running...");
        
        #[cfg(windows)]
        {
            unsafe {
                let mut msg: MSG = std::mem::zeroed();
                
                while self.running.load(Ordering::SeqCst) {
                    // Process messages
                    let ret = PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE);
                    
                    if ret != 0 {
                        TranslateMessage(&msg);
                        DispatchMessageW(&msg);
                        
                        // Check for quit
                        if msg.message == WM_QUIT {
                            break;
                        }
                    } else {
                        // No message - render frame
                        self.render_frame();
                        
                        // Frame timing - 60 FPS
                        std::thread::sleep(std::time::Duration::from_millis(16));
                    }
                    
                    if self.should_exit.load(Ordering::Relaxed) {
                        break;
                    }
                }
            }
        }
        
        #[cfg(not(windows))]
        {
            while self.running.load(Ordering::SeqCst) {
                std::thread::sleep(std::time::Duration::from_millis(16));
                if self.should_exit.load(Ordering::Relaxed) {
                    break;
                }
            }
        }
        
        // Cleanup
        unsafe {
            if let Some(state_ptr) = VORTEX_SHELL {
                let _ = Box::from_raw(state_ptr);
                VORTEX_SHELL = None;
            }
        }
        
        tracing::info!("VORTEX shell exited");
    }
    
    /// Render a frame - draws the taskbar
    fn render_frame(&mut self) {
        #[cfg(windows)]
        {
            if self.hwnd.is_null() { return; }
            
            unsafe {
                let mut ps: PAINTSTRUCT = std::mem::zeroed();
                let hdc = BeginPaint(self.hwnd, &mut ps);
                
                if hdc.is_null() { return; }
                
                // Background gradient
                let bg_brush = CreateSolidBrush(rgb(15, 20, 40));
                let mut rect = RECT {
                    left: 0,
                    top: 0,
                    right: self.screen_width,
                    bottom: 48,
                };
                FillRect(hdc, &rect, bg_brush);
                DeleteObject(bg_brush as *mut _);
                
                // Top glow line
                let glow_pen = CreatePen(PS_SOLID, 1, rgb(100, 150, 255));
                let old_pen = SelectObject(hdc, glow_pen as *mut _);
                MoveToEx(hdc, 0, 0, std::ptr::null_mut());
                LineTo(hdc, self.screen_width, 0);
                SelectObject(hdc, old_pen);
                DeleteObject(glow_pen as *mut _);
                
                // Bottom separator
                let sep_pen = CreatePen(PS_SOLID, 1, rgb(40, 50, 70));
                let old_pen2 = SelectObject(hdc, sep_pen as *mut _);
                MoveToEx(hdc, 0, 47, std::ptr::null_mut());
                LineTo(hdc, self.screen_width, 47);
                SelectObject(hdc, old_pen2);
                DeleteObject(sep_pen as *mut _);
                
                // Draw time
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default();
                let hours = (now.as_secs() / 3600) % 24;
                let mins = (now.as_secs() / 60) % 60;
                let time_str = format!("{:02}:{:02}", hours, mins);
                
                let time_wide: Vec<u16> = format!("{}\0", time_str).encode_utf16().collect();
                
                SetTextColor(hdc, rgb(200, 220, 255));
                SetBkMode(hdc, TRANSPARENT as i32);
                
                let mut time_rect = RECT {
                    left: self.screen_width - 80,
                    top: 15,
                    right: self.screen_width - 10,
                    bottom: 35,
                };
                DrawTextW(hdc, time_wide.as_ptr(), -1, &mut time_rect, DT_RIGHT | DT_VCENTER | DT_SINGLELINE);
                
                // Draw VORTEX logo
                let logo: Vec<u16> = "VORTEX\0".encode_utf16().collect();
                SetTextColor(hdc, rgb(100, 180, 255));
                let mut logo_rect = RECT {
                    left: 20,
                    top: 15,
                    right: 100,
                    bottom: 35,
                };
                DrawTextW(hdc, logo.as_ptr(), -1, &mut logo_rect, DT_LEFT | DT_VCENTER | DT_SINGLELINE);
                
                EndPaint(self.hwnd, &ps);
                
                // Update frame count
                if let Some(state_ptr) = VORTEX_SHELL {
                    let state = &mut *(state_ptr);
                    state.frame_count += 1;
                }
            }
        }
    }
    
    pub fn should_exit(&self) -> bool {
        self.should_exit.load(Ordering::Relaxed)
    }
    
    pub fn request_exit(&self) {
        self.should_exit.store(true, Ordering::SeqCst);
        self.running.store(false, Ordering::SeqCst);
    }
    
    pub fn handle_system_event(&self, _event: SystemEvent) {}
    
    pub fn get_screen_size(&self) -> (u32, u32) {
        (self.screen_width as u32, self.screen_height as u32)
    }
    
    pub fn register_hotkey(&self, _id: u32, _modifiers: u32, _key: u32) -> Result<()> {
        Ok(())
    }
    
    pub fn create_tray_icon(&self) -> Result<()> {
        Ok(())
    }
}

/// Window procedure for shell
#[cfg(windows)]
unsafe extern "system" fn shell_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: usize,
    lparam: isize
) -> LRESULT {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            if let Some(state_ptr) = VORTEX_SHELL {
                let state = &mut *(state_ptr);
                state.running = false;
            }
            return 0;
        }
        WM_CLOSE => {
            if let Some(state_ptr) = VORTEX_SHELL {
                let state = &mut *(state_ptr);
                state.running = false;
            }
            DestroyWindow(hwnd);
            return 0;
        }
        WM_PAINT => {
            return 0;
        }
        WM_ERASEBKGND => {
            return 1;
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam)
    }
}
