use dioxus::logger::tracing;
use serde::{Deserialize, Serialize};
use std::fs;
use std::env;
use std::io;
use std::path::{Path, PathBuf};
use toml;
use directories::{UserDirs, ProjectDirs};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub quick_access: Vec<PathBuf>,
    pub favourites: Vec<PathBuf>,
    pub tags: Vec<PathBuf>,
}

impl Default for AppConfig {
    fn default() -> Self {
        let mut quick_access = Vec::new();
        if let Some(user_dirs) = UserDirs::new() {
            let test = [
                user_dirs.desktop_dir(),
                user_dirs.document_dir(),
                user_dirs.download_dir(),
                user_dirs.video_dir(),
            ];
            quick_access.extend(test.iter().flatten().map(|p| p.to_path_buf()));
        }
        Self {
            quick_access,
            favourites: Vec::new(),
            tags: Vec::new(),
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, io::Error> {
        let config_path = Self::get_config_path();

        tracing::info!("Attempting to read from path: {}", config_path.to_string_lossy());
        if !config_path.exists() {
            tracing::warn!("Config path does not exist, creating default at: {}", config_path.to_string_lossy());
            let this= Self::default();
            this.save()?;
            return Ok(this);
        }

        // Read the config.toml file into memory
        let config_str = fs::read_to_string(&config_path)?;
        let config = toml::from_str(&config_str)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        tracing::info!("Config found and loaded");

        Ok(config)
    }

    pub fn save(&self) -> Result<(), io::Error> {
        let config_path = Self::get_config_path();
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let toml_string = toml::to_string(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        tracing::info!("Writing to path: {}", config_path.to_string_lossy());
        fs::write(config_path, toml_string)
    }

    pub fn add_favorite(&mut self, path: PathBuf) {
        if !self.favourites.contains(&path) {
            self.favourites.push(path);
        }
    }
    
    pub fn remove_favorite(&mut self, path: &Path) {
        self.favourites.retain(|p| p != path);
    }

    fn get_config_path() -> PathBuf {
        if let Some(proj_dirs) = ProjectDirs::from("", "", env!("CARGO_PKG_NAME")) {
            proj_dirs.config_dir().to_path_buf()
        } else {
            AppConfig::fallback_config_dir()
        }
            .join("config.toml")
    }

    fn fallback_config_dir() -> PathBuf {
        #[cfg(target_os = "linux")]
        {
            UserDirs::home_dir().map(|home| home.join("...")).unwrap()
        }
        #[cfg(target_os = "windows")]
        {
            UserDirs::data_dir().map(|data| data.join("...")).unwrap()
        }
        #[cfg(target_os = "macos")]
        {
            UserDirs::new()
                .unwrap_or_else(|| todo!("Display unknown home path error"))
                .home_dir()
                .join("Library/Application Support/")
        }
    }
}
