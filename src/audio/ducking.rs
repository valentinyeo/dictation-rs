use std::collections::HashMap;
use windows::core::*;
use windows::Win32::Media::Audio::*;
use windows::Win32::System::Com::*;

pub struct AudioDucker {
    sessions: HashMap<u32, (ISimpleAudioVolume, f32)>,
    duck_volume: f32,
    is_ducked: bool,
}

impl AudioDucker {
    pub fn new(duck_volume: f32) -> Self {
        Self {
            sessions: HashMap::new(),
            duck_volume,
            is_ducked: false,
        }
    }

    pub fn duck(&mut self) -> Result<()> {
        if self.is_ducked {
            return Ok(());
        }

        unsafe {
            CoInitialize(None)?;

            let enumerator: IMMDeviceEnumerator =
                CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;

            let device = enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?;

            let session_manager: IAudioSessionManager2 = device.Activate(CLSCTX_ALL, None)?;
            let session_enum = session_manager.GetSessionEnumerator()?;
            let count = session_enum.GetCount()?;

            self.sessions.clear();

            for i in 0..count {
                if let Ok(session) = session_enum.GetSession(i) {
                    let session2: IAudioSessionControl2 = session.cast()?;
                    let process_id = session2.GetProcessId()?;

                    // Skip our own process
                    if process_id == std::process::id() {
                        continue;
                    }

                    if let Ok(volume_control) = session.cast::<ISimpleAudioVolume>() {
                        let current_volume = volume_control.GetMasterVolume()?;

                        if current_volume > 0.01 {
                            // Store original volume and set to duck_volume
                            volume_control.SetMasterVolume(self.duck_volume, std::ptr::null())?;
                            self.sessions.insert(process_id, (volume_control, current_volume));
                        }
                    }
                }
            }

            CoUninitialize();
        }

        self.is_ducked = true;
        println!("[Ducker] Ducked {} audio sessions", self.sessions.len());
        Ok(())
    }

    pub fn restore(&mut self) -> Result<()> {
        if !self.is_ducked {
            return Ok(());
        }

        unsafe {
            CoInitialize(None)?;

            for (_, (volume_control, original_volume)) in &self.sessions {
                let _ = volume_control.SetMasterVolume(*original_volume, std::ptr::null());
            }

            CoUninitialize();
        }

        println!("[Ducker] Restored {} audio sessions", self.sessions.len());
        self.sessions.clear();
        self.is_ducked = false;
        Ok(())
    }
}

impl Drop for AudioDucker {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}
