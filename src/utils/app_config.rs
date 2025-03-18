use dioxus::logger::tracing;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::env;
use std::io;
use std::path::PathBuf;
use toml;
use directories::{UserDirs, ProjectDirs};

#[derive(Clone, Copy, PartialEq, Debug)]

pub enum Genre {
    Favourites,
    QuickAccess,
}

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
        let mut app_config = try_use_context::<Signal<AppConfig>>().expect("Could not find AppConfig context");
        let config_path = Self::get_config_path();
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let toml_string = toml::to_string(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        tracing::info!("Writing to path: {}", config_path.to_string_lossy());

        // Preferable solution would be to copy self directly into app_config, but seems I have to manually do each property instead.
        app_config.with_mut(|current_config| {
            current_config.favourites = self.favourites.clone();
            current_config.quick_access = self.quick_access.clone();
            current_config.tags = self.tags.clone();
        });

        fs::write(config_path, toml_string)
    }

    pub fn add_entry(&mut self, path: PathBuf, category: Genre) {
        let target_list = self.get_target_list(category);
        if !target_list.contains(&path) {
            target_list.push(path);
            self.save().expect("failed to save changes");
        }
    }

    pub fn remove_entry(&mut self, path: PathBuf, category: Genre) {
        let target_list = self.get_target_list(category);
        target_list.retain(|p| *p != path);
        self.save().expect("failed to save changes")
    }

    fn get_target_list(&mut self, category: Genre) -> &mut Vec<PathBuf> {
        return match category {
            Genre::Favourites => &mut self.favourites,
            Genre::QuickAccess => &mut self.quick_access,
        }
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
        // Implement the other operating system paths later
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
