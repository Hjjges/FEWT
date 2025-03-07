use dioxus::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::fs::DirEntry;

use chrono::{DateTime, Utc};
use crate::utils::{ModeContext, detect_extension};
use crate::components::{FolderIcon, FileIcon};

#[component]
pub fn FileExplorer(dir_path: String, level: usize) -> Element {

    // Setup
    let paths = get_paths(dir_path);
    let indent = calculate_indent(level);

    // Render the directories
    if *use_context::<ModeContext>().mode.read() {
        rsx! {
            div {
                style: "display: flex; flex-direction: column; gap: 20px; padding: 10px;",
                div {
                    style: "display: inline-flex; flex-wrap: wrap; gap: 50px; margin-top: 40px",
                    for path in paths {
                        if path.path().is_dir() {
                            FolderIcon { path: path.path().to_string_lossy().to_string() }
                        } else {
                            FileIcon { path: path.path().to_string_lossy().to_string() }
                        }
                    }
                }
            }
        }
    } else {
        rsx! {
            div {
                style: "display: flex; flex-direction: column; padding-top: {indent}px",

                if level == 0 {
                    div { style: "position: fixed; width: calc(100% - 260px); display: flex; background-color: #222; padding: 6px 6px; font-size: 14px; border-bottom: 2px solid #333", 
                        div { style: "display: inline-flex; width: 60%;", "Name" }
                        div { style: "display: inline-flex; width: 20%", "Type" }
                        div { style: "display: inline-flex; width; 20%", "Last Modified" }
                    }
                    div { style: "margin-top: 33px;"} // to combat the hidden first row
                }
                for path in paths {
                    if path.path().is_dir() {
                        Folder { path: path.path(), level: level }
                    } else {
                        File { path: path.path(), level: level }
                    }
                }
            }
        }
    }
}


#[component]
fn File(path: PathBuf, level: usize) -> Element { 
    let metadata = fs::metadata(&path)?;
    let mut date = "--".to_string();

    if let Ok(modified) = metadata.modified() {
        let datetime: DateTime<Utc> = modified.into();
        date = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    }
    rsx! {
        div {
            onclick: move |_| {
                // modify "focused element" state
            },
            style: "border-bottom: 4px solid #222; border-top: 2px solid #111; padding: 4px 4px 4px {level * 22 + 10}px; background-color: #334;",

            div { style: "font-size: 14px", 
                        div { style: "display: inline-flex; width: 60%", 
                            img { src: asset!("/assets/icons8-file.svg"), style: "width: 16px; height: 18px; padding-right: 8px;"}
                            "{path.file_name().unwrap().to_string_lossy()}" 
                        }
                        if let Some(ext) = path.extension() {
                            div { style: "display: inline-flex; width: 20%", "{detect_extension(ext.to_string_lossy().to_string())} " }
                        } else {
                            div { style: "display: inline-flex; width: 20%", "File" }
                        }
                        div { style: "display: inline-flex; width; 20%", "{date}" }
            }
        }
    }
}

#[component]
fn Folder(path: PathBuf, level: usize) -> Element {
    let mut is_expanded = use_signal(|| false);
    let metadata = fs::metadata(&path)?;
    let mut date = "--".to_string();

    if let Ok(modified) = metadata.modified() {
        let datetime: DateTime<Utc> = modified.into();
        date = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    }

    rsx! {
        div {
            onclick: move |_| { 
                is_expanded.toggle();
                // modify "focus element" state
             },
            style: "border-bottom: 4px solid #222; border-top: 2px solid #111; padding: 4px 4px 4px {level * 22 + 10}px; background-color: #334;",

            div { style: "font-size: 14px;", 
                        div { style: "display: inline-flex; width: 60%;", 
                            img { src: asset!("/assets/folder-svgrepo-com.svg"), style: "width: 16px; height: 18px; padding-right: 8px;"}
                            "{path.file_name().unwrap().to_string_lossy()}" 
                        }
                        div { style: "display: inline-flex; width: 20%", "Folder" }
                        div { style: "display: inline-flex; width; 20%", "{date}" }
            }
        }
        if *is_expanded.read() {
            FileExplorer { dir_path: path.to_string_lossy(), level: level + 1 } // Recursive call on FileExplorer
        }
    }
}

fn calculate_indent(level: usize) -> i32 {
    if level == 0 {
        return 40;
    } else {
        return 0;
    }
}

fn get_paths(dir_path: String) -> Vec<DirEntry> {
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