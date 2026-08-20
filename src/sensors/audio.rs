use std::collections::HashSet;
use windows::core::Interface;
use windows::Win32::System::Com::*;
use windows::Win32::Media::Audio::*;

pub struct AudioSensor {
    com_initialized: bool,
}

impl AudioSensor {
    pub fn new() -> Self {
        let result = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };
        let com_initialized = result.is_ok() || result == windows::core::HRESULT(1);
        Self { com_initialized }
    }

    pub fn get_active_audio_pids(&self) -> HashSet<u32> {
        let mut pids = HashSet::new();
        if !self.com_initialized {
            return pids;
        }

        unsafe {
            let enumerator_res: Result<IMMDeviceEnumerator, _> =
                CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL);

            if let Ok(enumerator) = enumerator_res {
                for &direction in &[eRender, eCapture] {
                    if let Ok(dev) = enumerator.GetDefaultAudioEndpoint(direction, eConsole) {
                        if let Ok(manager) = dev.Activate::<IAudioSessionManager2>(CLSCTX_ALL, None) {
                            if let Ok(session_enum) = manager.GetSessionEnumerator() {
                                if let Ok(count) = session_enum.GetCount() {
                                    for i in 0..count {
                                        if let Ok(ctrl) = session_enum.GetSession(i) {
                                            if let Ok(state) = ctrl.GetState() {
                                                if state == AudioSessionStateActive {
                                                    if let Ok(ctrl2) = ctrl.cast::<IAudioSessionControl2>() {
                                                        if let Ok(pid) = ctrl2.GetProcessId() {
                                                            if pid > 0 {
                                                                pids.insert(pid);
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        pids
    }
}

impl Drop for AudioSensor {
    fn drop(&mut self) {
        if self.com_initialized {
            unsafe { CoUninitialize() };
        }
    }
}
