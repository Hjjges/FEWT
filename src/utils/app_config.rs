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
        let config_path = Self::get_config_path()?;

        if !config_path.exists() {
            println!("Config path does not exist");
            return Ok(Self::default());
        }

        // Read the config.toml file into memory
        let config_str = fs::read_to_string(&config_path)?;
        let config = toml::from_str(&config_str)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;


        Ok(config)
    }

    pub fn save(&self) -> Result<(), io::Error> {
        let config_path = Self::get_config_path()?;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let toml_string = toml::to_string(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

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

    fn get_config_path() -> Result<PathBuf, io::Error> {
        let username = env::var("USERNAME")
            .or_else(|_| env::var("USER"))
            .unwrap_or_else(|_| "user".to_string());

        let config_path = if let Some(proj_dirs) = ProjectDirs::from("", &username, "rust_file_explorer") {
            proj_dirs.config_dir().to_path_buf()
        } else {
            let fallback_path = fallback_config_dir();
            if let Err(e) = fs::create_dir_all(&fallback_path) {
                eprintln!("Failed to create config directory: {}", e);
            } else {
                println!("Created fallback config directory at {:?}", fallback_path);
            }
            Ok(fallback_path)
        };
    

        let config_dir = proj_dirs.config_dir();
        Ok(config_dir.join("config.toml"))
    }
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
        UserDirs::home_dir()
            .map(|home| home.join("Library/Application Support/..."))
            .unwrap()
    }
}