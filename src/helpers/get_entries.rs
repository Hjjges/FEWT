use std::fs;
use crate::utils::FileEntry;

pub fn get_entries(dir_path: String) -> Result<Vec<FileEntry>, std::io::Error> {
    let entries = fs::read_dir(&dir_path)?;
    let mut files: Vec<FileEntry> = entries
        .filter_map(|entry| FileEntry::new(entry.ok()?).ok())
        .collect();

    files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(files)
}