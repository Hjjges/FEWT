use std::fs;
use std::fs::DirEntry;

use dioxus::logger::tracing;

pub fn get_paths(dir_path: String) -> Vec<DirEntry> {
    return match fs::read_dir(&dir_path) {
        Ok(entries) => {
            match entries.collect::<Result<Vec<_>, std::io::Error>>() {
                Ok(paths) => paths,
                Err(e) => {
                    tracing::error!("Error collecting directory entries: {}", e);
                    return Vec::new()
                }
            }
        },
        Err(e) => {
            tracing::error!("Error reading directory '{}': {}", dir_path, e);
            return Vec::new()
        }
    };
}