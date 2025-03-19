use std::fs;
use std::path::PathBuf;
use crate::utils::extension_detection::{detect_extension, FileType};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub path_string: String,
    pub is_dir: bool,
    pub modified: String,
    pub extension: FileType,
}

impl FileEntry {
    pub fn new(entry: fs::DirEntry) -> std::io::Result<Self> {
        let metadata = entry.metadata()?;
        let mut date = "--".to_string();
        if let Ok(modified) = metadata.modified() {
            let datetime: DateTime<Utc> = modified.into();
            date = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        }

        let extension: FileType;
        if let Some(result) = entry.path().extension() {
            extension = detect_extension(result.to_string_lossy().to_string());
        } else {
            extension = FileType::None;
        }

        Ok(Self {
            name: entry.file_name().to_string_lossy().into_owned(),
            path: entry.path(),
            path_string: entry.path().to_string_lossy().to_string(),
            is_dir: metadata.is_dir(),
            modified: date,
            extension,
        })
    }
    pub fn new_file(file_path: String, metadata: fs::Metadata) -> std::io::Result<Self> {
        let mut date = "--".to_string();
        if let Ok(modified) = metadata.modified() {
            let datetime: DateTime<Utc> = modified.into();
            date = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        }
        let path =  PathBuf::from(&file_path);
        let extension: FileType;
        if let Some(result) = path.extension() {
            extension = detect_extension(result.to_string_lossy().to_string());
        } else {
            extension = FileType::None;
        }

        Ok(Self {
            name: path.file_name().unwrap().to_string_lossy().into_owned(),
            path,
            path_string: file_path,
            is_dir: false,
            modified: date,
            extension
        })
    }
}
