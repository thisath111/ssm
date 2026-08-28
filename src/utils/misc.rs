use std::mem;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::Security::*;
use windows::Win32::System::Power::*;
use windows::Win32::System::Threading::*;
use windows::Win32::UI::WindowsAndMessaging::*;

#[inline]
pub fn open_process(pid: u32, access: PROCESS_ACCESS_RIGHTS) -> Option<HANDLE> {
    if pid == 0 {
        return None;
    }
    unsafe { OpenProcess(access, false, pid).ok() }
}

#[inline]
pub fn close_handle(h: HANDLE) {
    if !h.is_invalid() {
        let _ = unsafe { CloseHandle(h) };
    }
}

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

#[inline]
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

#[inline]
pub fn get_foreground_pid(hwnd: HWND) -> u32 {
    unsafe {
        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        pid
    }
}

#[inline]
pub fn is_window_hung(hwnd: HWND) -> bool {
    unsafe { IsHungAppWindow(hwnd).as_bool() }
}

pub fn is_on_battery() -> bool {
    unsafe {
        let mut status = SYSTEM_POWER_STATUS::default();
        if GetSystemPowerStatus(&mut status).is_ok() {
            status.ACLineStatus == 0
        } else {
            false
        }
    }
}

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

#[allow(dead_code)]
#[inline]
pub fn disable_priority_boost(pid: u32) {
    if let Some(h) = open_process(pid, PROCESS_SET_INFORMATION) {
        unsafe {
            let _ = SetProcessPriorityBoost(h, true);
        }
        close_handle(h);
    }
}

#[inline]
pub fn enable_priority_boost(pid: u32) {
    if let Some(h) = open_process(pid, PROCESS_SET_INFORMATION) {
        unsafe {
            let _ = SetProcessPriorityBoost(h, false);
        }
        close_handle(h);
    }
}

#[inline]
pub fn set_process_io_priority(pid: u32, priority: u32) {
    if let Some(h) = open_process(pid, PROCESS_SET_INFORMATION) {
        unsafe {
            let _ = SetProcessInformation(
                h,
                PROCESS_INFORMATION_CLASS(9),
                &priority as *const _ as *const _,
                std::mem::size_of::<u32>() as u32,
            );
        }
        close_handle(h);
    }
}

#[inline]
pub fn set_process_memory_priority(pid: u32, priority: u32) {
    if let Some(h) = open_process(pid, PROCESS_SET_INFORMATION) {
        unsafe {
            let _ = SetProcessInformation(
                h,
                ProcessMemoryPriority,
                &priority as *const _ as *const _,
                std::mem::size_of::<u32>() as u32,
            );
        }
        close_handle(h);
    }
}
