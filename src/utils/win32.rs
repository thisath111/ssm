use std::mem;
use windows::Win32::Foundation::*;
use windows::Win32::System::Threading::*;
use windows::Win32::Security::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Graphics::Gdi::*;

/// Checks if current process is running with Administrator privileges.
pub fn is_elevated() -> bool {
    unsafe {
        let mut token = HANDLE::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_err() {
            return false;
        }
        let mut elevation = TOKEN_ELEVATION::default();
        let mut size = mem::size_of::<TOKEN_ELEVATION>() as u32;
        let res = GetTokenInformation(
            token,
            TokenElevation,
            Some(&mut elevation as *mut _ as *mut _),
            size,
            &mut size,
        );
        let _ = CloseHandle(token);
        res.is_ok() && elevation.TokenIsElevated != 0
    }
}

/// Enables specified privilege (e.g., SeDebugPrivilege, SeIncreaseBasePriorityPrivilege) for process token.
pub fn enable_privilege(privilege_name: &str) -> bool {
    unsafe {
        let mut token = HANDLE::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &mut token).is_err() {
            return false;
        }

        let mut luid = LUID::default();
        let name_u16: Vec<u16> = privilege_name.encode_utf16().chain(std::iter::once(0)).collect();
        if windows::Win32::Security::LookupPrivilegeValueW(None, windows::core::PCWSTR(name_u16.as_ptr()), &mut luid).is_err() {
            let _ = CloseHandle(token);
            return false;
        }

        let mut tp = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            Privileges: [LUID_AND_ATTRIBUTES {
                Luid: luid,
                Attributes: SE_PRIVILEGE_ENABLED,
            }],
        };

        let res = AdjustTokenPrivileges(token, false, Some(&mut tp), 0, None, None);
        let _ = CloseHandle(token);
        res.is_ok()
    }
}

/// Safely opens a handle to a process with requested access rights.
pub fn open_process(pid: u32, access: PROCESS_ACCESS_RIGHTS) -> Option<HANDLE> {
    if pid <= 4 {
        return None;
    }
    unsafe { OpenProcess(access, false, pid).ok() }
}

/// Closes an open Win32 handle.
pub fn close_handle(h: HANDLE) {
    if !h.is_invalid() {
        let _ = unsafe { CloseHandle(h) };
    }
}

/// Returns HWND of current foreground active window.
pub fn get_foreground_hwnd() -> Option<HWND> {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.is_invalid() || hwnd.0.is_null() {
            None
        } else {
            Some(hwnd)
        }
    }
}

/// Returns process ID for given HWND.
pub fn get_process_id_from_hwnd(hwnd: HWND) -> u32 {
    unsafe {
        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        pid
    }
}

/// Checks if window is currently unresponsive / hung.
pub fn is_window_hung(hwnd: HWND) -> bool {
    unsafe { IsHungAppWindow(hwnd).as_bool() }
}

/// Checks if window occupies full monitor workspace (game/video player).
pub fn is_fullscreen(hwnd: HWND) -> bool {
    unsafe {
        let mut win_rect = RECT::default();
        if GetWindowRect(hwnd, &mut win_rect).is_err() {
            return false;
        }

        let hmon = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
        if hmon.is_invalid() {
            return false;
        }

        let mut info = MONITORINFO {
            cbSize: mem::size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        if !GetMonitorInfoW(hmon, &mut info).as_bool() {
            return false;
        }

        let m = info.rcMonitor;
        win_rect.left <= m.left
            && win_rect.top <= m.top
            && win_rect.right >= m.right
            && win_rect.bottom >= m.bottom
    }
}

/// Registers current executable directory into Windows User Environment PATH registry key.
pub fn register_in_path() -> Result<bool, Box<dyn std::error::Error>> {
    use winreg::enums::*;
    use winreg::RegKey;

    let exe_path = std::env::current_exe()?;
    let exe_dir = match exe_path.parent() {
        Some(dir) => dir,
        None => return Ok(false),
    };
    let dir_str = match exe_dir.to_str() {
        Some(s) => s,
        None => return Ok(false),
    };

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)?;
    let current_path: String = env.get_value("Path").unwrap_or_default();

    let paths: Vec<&str> = current_path.split(';').collect();
    if paths.iter().any(|&p| p.trim().eq_ignore_ascii_case(dir_str)) {
        return Ok(false);
    }

    let new_path = if current_path.trim().is_empty() {
        dir_str.to_string()
    } else {
        format!("{};{}", current_path.trim_end_matches(';'), dir_str)
    };

    env.set_value("Path", &new_path)?;

    unsafe {
        let env_str: Vec<u16> = "Environment\0".encode_utf16().collect();
        let _ = SendMessageTimeoutW(
            HWND_BROADCAST,
            WM_SETTINGCHANGE,
            WPARAM(0),
            LPARAM(env_str.as_ptr() as isize),
            SMTO_ABORTIFHUNG,
            1000,
            None,
        );
    }

    Ok(true)
}
