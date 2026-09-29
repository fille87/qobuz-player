use std::path::{Path, PathBuf};

use crate::{AppResult, AudioQuality, error::PlayerError};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(default)]
pub struct Config {
    pub max_audio_quality: AudioQuality,
    pub use_file_based_streaming: bool,
    pub cache_directory: PathBuf,
    pub cache_ttl_hours: u32,
    pub enable_disconnect: bool,
    pub disconnect_server_url: Option<String>,
    pub disconnect_password: Option<String>,
    pub device_name: Option<String>, // TODO: For qobuz disconnect, change name?
    pub auto_play: bool,
    pub play_entire_list: bool, // play only track on play selected track (not continue with the remaining tracks from list)
    pub theme: Theme,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(default)]
pub struct Theme {
    colour: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            colour: String::from("blue"),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_audio_quality: Default::default(),
            use_file_based_streaming: false,
            cache_directory: {
                let mut cache_dir = std::env::temp_dir();
                cache_dir.push("qobine-cache");
                cache_dir
            },
            cache_ttl_hours: 1,
            enable_disconnect: false,
            disconnect_server_url: None,
            disconnect_password: None,
            device_name: None,
            auto_play: false,
            play_entire_list: true,
            theme: Default::default(),
        }
    }
}

impl Config {
    pub fn read_from_file(path: &Path) -> AppResult<Self> {
        let file = std::fs::read_to_string(path).or_else(|e| {
            Err(PlayerError::ConfigError {
                message: format!(
                    "error while trying to open config file '{}' - {}",
                    path.display(),
                    e
                ),
            })
        })?;
        let cfg: Config = toml::from_str(&file).or_else(|e| {
            Err(PlayerError::ConfigError {
                message: e.to_string(),
            })
        })?;
        Ok(cfg)
    }
}
