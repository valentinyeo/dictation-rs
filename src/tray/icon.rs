use crate::config::Config;
use crate::state::{AppState, StateManager};
use std::sync::Arc;
use tray_icon::menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem, CheckMenuItem};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};
use windows::Win32::System::Registry::*;
use windows::Win32::Foundation::*;
use windows::core::*;

pub struct TrayManager {
    tray_icon: TrayIcon,
    menu: Menu,
    pause_item: MenuItem,
    language_item: MenuItem,
    autostart_item: CheckMenuItem,
    state_manager: Arc<StateManager>,
    config: Arc<tokio::sync::RwLock<Config>>,
}

impl TrayManager {
    pub fn new(
        state_manager: Arc<StateManager>,
        config: Arc<tokio::sync::RwLock<Config>>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let menu = Menu::new();

        let pause_item = MenuItem::new("Pause", true, None);
        let language_item = MenuItem::new("Language: English", true, None);
        let autostart_item = CheckMenuItem::new("Start on Boot", true, Self::is_autostart_enabled(), None);
        let settings_item = MenuItem::new("Settings", true, None);
        let exit_item = MenuItem::new("Exit", true, None);

        menu.append(&pause_item)?;
        menu.append(&PredefinedMenuItem::separator())?;
        menu.append(&language_item)?;
        menu.append(&autostart_item)?;
        menu.append(&PredefinedMenuItem::separator())?;
        menu.append(&settings_item)?;
        menu.append(&exit_item)?;

        // Load grey icon initially (Active state)
        let icon = Self::load_icon("grey")?;

        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(menu.clone()))
            .with_tooltip("Dictation App")
            .with_icon(icon)
            .build()?;

        Ok(Self {
            tray_icon,
            menu,
            pause_item,
            language_item,
            autostart_item,
            state_manager,
            config,
        })
    }

    pub async fn handle_events(&mut self) {
        if let Ok(event) = MenuEvent::receiver().try_recv() {
            if event.id == self.pause_item.id() {
                self.toggle_pause().await;
            } else if event.id == self.language_item.id() {
                self.toggle_language().await;
            } else if event.id == self.autostart_item.id() {
                self.toggle_autostart().await;
            } else if event.id.0.contains("Settings") {
                let _ = Config::open_in_editor();
            } else if event.id.0.contains("Exit") {
                std::process::exit(0);
            }
        }
    }

    pub async fn update_icon(&mut self, state: AppState) {
        let color = state.icon_color();
        if let Ok(icon) = Self::load_icon(color) {
            let _ = self.tray_icon.set_icon(Some(icon));
        }

        // Update pause menu item text
        let pause_text = if matches!(state, AppState::Paused) {
            "Resume"
        } else {
            "Pause"
        };
        let _ = self.pause_item.set_text(pause_text);
    }

    async fn toggle_pause(&mut self) {
        let new_state = self.state_manager.toggle_pause().await;
        self.update_icon(new_state).await;
    }

    async fn toggle_language(&mut self) {
        let mut config = self.config.write().await;
        config.deepgram.language = if config.deepgram.language == "en" {
            "de".to_string()
        } else {
            "en".to_string()
        };

        let lang_text = if config.deepgram.language == "en" {
            "Language: English"
        } else {
            "Language: German"
        };
        let _ = self.language_item.set_text(lang_text);

        // Save config
        let _ = config.save();
        println!("[Tray] Language switched to: {}", config.deepgram.language);
    }

    async fn toggle_autostart(&mut self) {
        let enabled = self.autostart_item.is_checked();
        let new_state = !enabled;

        if let Err(e) = Self::set_autostart(new_state) {
            eprintln!("[Tray] Failed to set autostart: {}", e);
            return;
        }

        self.autostart_item.set_checked(new_state);
        println!("[Tray] Autostart {}", if new_state { "enabled" } else { "disabled" });
    }

    fn load_icon(color: &str) -> Result<Icon, Box<dyn std::error::Error>> {
        // For now, use a simple colored icon
        // In production, load from resources/mic-{color}.ico
        let icon_data = Self::generate_icon(color);
        Ok(Icon::from_rgba(icon_data, 32, 32)?)
    }

    fn generate_icon(color: &str) -> Vec<u8> {
        // Generate a simple 32x32 RGBA icon
        let mut data = vec![0u8; 32 * 32 * 4];

        let (r, g, b) = match color {
            "blue" => (0, 120, 212),   // #0078D4
            "red" => (209, 52, 56),    // #D13438
            _ => (138, 138, 138),      // grey #8A8A8A
        };

        // Draw a simple circle (microphone representation)
        for y in 0..32 {
            for x in 0..32 {
                let dx = x as f32 - 16.0;
                let dy = y as f32 - 16.0;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist < 12.0 {
                    let idx = (y * 32 + x) * 4;
                    data[idx] = r;
                    data[idx + 1] = g;
                    data[idx + 2] = b;
                    data[idx + 3] = 255;
                }
            }
        }

        data
    }

    fn is_autostart_enabled() -> bool {
        unsafe {
            let key_path = w!("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
            let app_name = w!("DictationApp");

            let mut key = HKEY::default();
            if RegOpenKeyExW(HKEY_CURRENT_USER, key_path, 0, KEY_READ, &mut key).is_ok() {
                let mut buffer = [0u16; 512];
                let mut buffer_size = (buffer.len() * 2) as u32;

                let result = RegQueryValueExW(
                    key,
                    app_name,
                    None,
                    None,
                    Some(buffer.as_mut_ptr() as *mut u8),
                    Some(&mut buffer_size),
                );

                let _ = RegCloseKey(key);
                return result.is_ok();
            }
        }
        false
    }

    fn set_autostart(enable: bool) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let key_path = w!("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
            let app_name = w!("DictationApp");

            let mut key = HKEY::default();

            if enable {
                // Open or create the key
                RegOpenKeyExW(HKEY_CURRENT_USER, key_path, 0, KEY_SET_VALUE, &mut key)
                    .map_err(|e| format!("Failed to open registry key: {:?}", e))?;

                let exe_path = std::env::current_exe()
                    .map_err(|e| format!("Failed to get exe path: {}", e))?;
                let path_str = exe_path.to_string_lossy();
                let mut wide: Vec<u16> = path_str.encode_utf16().collect();
                wide.push(0);

                // Convert u16 slice to u8 slice for the registry
                let byte_slice = std::slice::from_raw_parts(
                    wide.as_ptr() as *const u8,
                    wide.len() * 2,
                );

                RegSetValueExW(
                    key,
                    app_name,
                    0,
                    REG_SZ,
                    Some(byte_slice),
                ).map_err(|e| format!("Failed to set registry value: {:?}", e))?;

                RegCloseKey(key)
                    .map_err(|e| format!("Failed to close registry key: {:?}", e))?;
            } else {
                if RegOpenKeyExW(HKEY_CURRENT_USER, key_path, 0, KEY_SET_VALUE, &mut key).is_ok() {
                    let _ = RegDeleteValueW(key, app_name);
                    let _ = RegCloseKey(key);
                }
            }
        }

        Ok(())
    }
}
