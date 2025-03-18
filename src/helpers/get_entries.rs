use std::fs;

use crate::utils::FileEntry;
use crate::SORT;


pub fn get_entries(dir_path: String) -> Result<Vec<FileEntry>, std::io::Error> {
    let entries = fs::read_dir(&dir_path)?;
    let mut files: Vec<FileEntry> = entries
        .filter_map(|entry| FileEntry::new(entry.ok()?).ok())
        .collect();

    // SORT is a string, so just add - before the string. e.g.: -extension, -name
    let sort = SORT().clone();
    let is_descending = sort.starts_with('-');
    let clean_sort = if is_descending { sort.trim_start_matches('-') } else { sort.as_str() };
    match clean_sort {
        "name" => files.sort_by(|a, b| {
            if is_descending {
                b.name.to_lowercase().cmp(&a.name.to_lowercase())
            } else {
                a.name.to_lowercase().cmp(&b.name.to_lowercase())
            }
        }),
        "extension" => files.sort_by(|a, b| {
            let x = match is_descending { true => b, false => a };
            let y = match is_descending { true => a, false => b };
            b.is_dir.cmp(&a.is_dir)
                    .then_with(|| x.extension.to_string().to_lowercase().cmp(&y.extension.to_string().to_lowercase()))
                    .then_with(|| x.name.to_lowercase().cmp(&y.name.to_lowercase()))
        }),
        "modified" => files.sort_by(|a, b| {
        if is_descending {
            b.modified.to_lowercase().cmp(&a.modified.to_lowercase())
        } else {
            a.modified.to_lowercase().cmp(&b.modified.to_lowercase())
        }
    }),
        _ => {}
    }

    Ok(files)
}