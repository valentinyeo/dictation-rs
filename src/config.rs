use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub deepgram: DeepgramConfig,
    pub audio: AudioConfig,
    pub vad: VadConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepgramConfig {
    pub api_key: String,
    pub language: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub silence_threshold_ms: u64,
    pub duck_volume: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VadConfig {
    pub energy_threshold: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            deepgram: DeepgramConfig {
                api_key: String::new(),
                language: "en".to_string(),
                model: "nova-2".to_string(),
            },
            audio: AudioConfig {
                silence_threshold_ms: 3000,
                duck_volume: 0.2,
            },
            vad: VadConfig {
                energy_threshold: 0.02,
            },
        }
    }
}

impl Config {
    pub fn load_or_create() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            config.validate()?;
            Ok(config)
        } else {
            // Create config directory if it doesn't exist
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // Create default config with comments
            let default_config = Self::default();
            let toml_string = toml::to_string_pretty(&default_config)?;

            let commented = format!(
                "# Dictation App Configuration\n\
                 # Edit this file to configure the application\n\
                 # Get your Deepgram API key from: https://console.deepgram.com/\n\n\
                 {}\n\n\
                 # Audio settings\n\
                 # silence_threshold_ms: Auto-pause after this many milliseconds of silence\n\
                 # duck_volume: Volume level (0.0-1.0) for other apps during dictation\n\n\
                 # VAD settings\n\
                 # energy_threshold: RMS threshold for speech detection (0.0-1.0)\n",
                toml_string
            );

            fs::write(&config_path, commented)?;
            Ok(default_config)
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;
        let toml_string = toml::to_string_pretty(self)?;
        fs::write(config_path, toml_string)?;
        Ok(())
    }

    pub fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.deepgram.api_key.is_empty() {
            return Err("API key not set. Please edit config.toml and add your Deepgram API key.".into());
        }
        Ok(())
    }

    pub fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = directories::BaseDirs::new()
            .ok_or("Failed to get base directories")?
            .config_dir()
            .join("dictation");
        Ok(config_dir.join("config.toml"))
    }

    pub fn open_in_editor() -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;
        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("cmd")
                .args(&["/C", "start", config_path.to_str().unwrap()])
                .spawn()?;
        }
        Ok(())
    }
}
