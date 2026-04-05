#[derive(Debug, Clone)]
pub struct WindowSample {
    pub id: String,
    pub title: String,
    pub is_focused: bool,
}

#[cfg(windows)]
mod imp {
    use super::WindowSample;
    use windows_sys::Win32::Foundation::{BOOL, HWND, LPARAM, WPARAM};
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        EnumWindows, GetForegroundWindow, GetWindowTextLengthW, GetWindowTextW, IsWindowVisible,
        PostMessageW, SetForegroundWindow, ShowWindow, SW_MAXIMIZE, SW_MINIMIZE, WM_CLOSE,
    };

    pub fn capture_windows() -> Vec<WindowSample> {
        let mut items: Vec<WindowSample> = Vec::new();
        let ptr = &mut items as *mut Vec<WindowSample>;

        unsafe {
            let _ = EnumWindows(Some(enum_windows_proc), ptr as LPARAM);
        }

        items
    }

    pub fn perform_action(window_id: &str, action: &str) -> Result<(), String> {
        let hwnd = parse_hwnd(window_id)?;

        unsafe {
            match action {
                "focus" => {
                    if SetForegroundWindow(hwnd) == 0 {
                        return Err(format!("failed to focus window {}", window_id));
                    }
                }
                "minimize" => {
                    ShowWindow(hwnd, SW_MINIMIZE);
                }
                "maximize" => {
                    ShowWindow(hwnd, SW_MAXIMIZE);
                }
                "close" => {
                    PostMessageW(hwnd, WM_CLOSE, 0 as WPARAM, 0);
                }
                other => return Err(format!("unsupported window action `{}`", other)),
            }
        }

        Ok(())
    }

    unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        if IsWindowVisible(hwnd) == 0 {
            return 1;
        }

        let len = GetWindowTextLengthW(hwnd);
        if len <= 0 {
            return 1;
        }

        let mut buffer = vec![0u16; (len + 1) as usize];
        let written = GetWindowTextW(hwnd, buffer.as_mut_ptr(), len + 1);
        if written <= 0 {
            return 1;
        }

        let title = String::from_utf16_lossy(&buffer[..written as usize])
            .trim()
            .to_string();
        if title.is_empty() || title == "Program Manager" {
            return 1;
        }

        let focused = GetForegroundWindow() == hwnd;
        let id = format!("hwnd-{}", hwnd as isize);

        let list = &mut *(lparam as *mut Vec<WindowSample>);
        list.push(WindowSample {
            id,
            title,
            is_focused: focused,
        });

        1
    }

    fn parse_hwnd(window_id: &str) -> Result<HWND, String> {
        let raw = window_id
            .strip_prefix("hwnd-")
            .ok_or_else(|| format!("invalid window id `{}`", window_id))?;

        let value = raw
            .parse::<isize>()
            .map_err(|_| format!("invalid window id `{}`", window_id))?;

        Ok(value as HWND)
    }
}

#[cfg(not(windows))]
mod imp {
    use super::WindowSample;

    pub fn capture_windows() -> Vec<WindowSample> {
        Vec::new()
    }

    pub fn perform_action(_window_id: &str, _action: &str) -> Result<(), String> {
        Ok(())
    }
}

pub use imp::{capture_windows, perform_action};
