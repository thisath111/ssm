// audio_stream.rs
// BUG FIX: COM is now initialized ONCE in new() not every 500ms tick.
// Monitors active audio playback and recording streams via Windows WASAPI.
// Used to protect communication apps (Discord, Zoom, Teams) from suspension.

use std::collections::HashSet;
use windows::core::Interface;
use windows::Win32::System::Com::*;
use windows::Win32::Media::Audio::*;
use windows::Win32::Media::Audio::Endpoints::*;
use log::warn;

pub struct AudioStreamSensor {
    com_initialized: bool,
}

impl AudioStreamSensor {
    pub fn new() -> Self {
        // BUG FIX: Initialize COM once here rather than on every tick.
        // CoInitializeEx returns S_FALSE if already initialized on this thread — that is fine.
        let result = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };
        let com_initialized = result.is_ok() || result == windows::core::HRESULT(1); // S_FALSE = 1

        Self { com_initialized }
    }

    /// Queries all active audio render (playback) and capture (recording/microphone) session PIDs.
    pub fn get_active_audio_pids(&self) -> HashSet<u32> {
        let mut active_pids = HashSet::new();

        if !self.com_initialized {
            return active_pids;
        }

        unsafe {
            let enumerator_res: Result<IMMDeviceEnumerator, _> =
                CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL);

            let enumerator = match enumerator_res {
                Ok(e) => e,
                Err(e) => {
                    warn!("[AudioStream] Failed to create MMDeviceEnumerator: {:?}", e);
                    return active_pids;
                }
            };

            // Check both playback (eRender) and microphone/capture (eCapture) endpoints
            for &direction in &[eRender, eCapture] {
                if let Ok(dev) = enumerator.GetDefaultAudioEndpoint(direction, eConsole) {
                    if let Ok(manager) = dev.Activate::<IAudioSessionManager2>(CLSCTX_ALL, None) {
                        if let Ok(session_enum) = manager.GetSessionEnumerator() {
                            if let Ok(count) = session_enum.GetCount() {
                                for i in 0..count {
                                    if let Ok(session_ctrl) = session_enum.GetSession(i) {
                                        if let Ok(state) = session_ctrl.GetState() {
                                            if state == AudioSessionStateActive {
                                                if let Ok(sc2) =
                                                    session_ctrl.cast::<IAudioSessionControl2>()
                                                {
                                                    if let Ok(pid) = sc2.GetProcessId() {
                                                        if pid != 0 {
                                                            active_pids.insert(pid);
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

        active_pids
    }
}

impl Drop for AudioStreamSensor {
    fn drop(&mut self) {
        if self.com_initialized {
            unsafe { CoUninitialize() };
        }
    }
}
