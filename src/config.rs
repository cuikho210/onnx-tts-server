use dirs;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TtsConfig {
    pub model: String,
    pub tokens: String,
    pub lexicon: Option<String>,
    pub espeak_ng_data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub tts: TtsConfig,
}
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 3001,
            },
            tts: TtsConfig {
                model: String::new(),
                tokens: String::new(),
                lexicon: Some(String::new()),
                espeak_ng_data: Some(String::new()),
            },
        }
    }
}
impl AppConfig {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: AppConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn default_path() -> Option<std::path::PathBuf> {
        if let Some(home) = dirs::home_dir() {
            match env::consts::OS {
                "linux" | "macos" => Some(home.join(".config/onnx-tts-server/config.toml")),
                "windows" => Some(home.join("AppData/Roaming/onnx-tts-server/config.toml")),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn load_config() -> Self {
        if let Some(path) = Self::default_path() {
            if path.exists() {
                return Self::from_file(&path).unwrap_or_default();
            }
        }
        Self::default()
    }
}
