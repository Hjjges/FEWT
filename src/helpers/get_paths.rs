use std::fs;
use std::fs::DirEntry;

pub fn get_paths(dir_path: String) -> Vec<DirEntry> {
    return match fs::read_dir(&dir_path) {
        Ok(entries) => {
            match entries.collect::<Result<Vec<_>, std::io::Error>>() {
                Ok(paths) => paths,
                Err(e) => {
                    println!("Error collecting directory entries: {}", e);
                    return Vec::new()
                }
            }
        },
        Err(e) => {
            println!("Error reading directory '{}': {}", dir_path, e);
            return Vec::new()
        }
    };
}