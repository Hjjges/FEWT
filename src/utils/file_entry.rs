use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub path_string: String,
    pub is_dir: bool,
    pub modified: String,
}

impl FileEntry {
    pub fn new(entry: fs::DirEntry) -> std::io::Result<Self> {
        let metadata = entry.metadata()?;
        let mut date = "--".to_string();
        if let Ok(modified) = metadata.modified() {
            let datetime: DateTime<Utc> = modified.into();
            date = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        }

        Ok(Self {
            name: entry.file_name().to_string_lossy().into_owned(),
            path: entry.path(),
            path_string: entry.path().to_string_lossy().to_string(),
            is_dir: metadata.is_dir(),
            modified: date,
        })
    }
}
