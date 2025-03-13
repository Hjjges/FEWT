use dioxus::prelude::*;
use std::path::PathBuf;
use std::fs;
use chrono::{DateTime, Utc};

use crate::utils::detect_extension;
use crate::helpers::get_paths;
use crate::components::FileExplorer;

const IMG_FILE: Asset = asset!("/assets/icons8-file.svg");
const IMG_FOLDER: Asset = asset!("/assets/folder-svgrepo-com.svg");

#[component]
pub fn ListFileExplorer(dir_path: String, level: usize) -> Element {
    let paths = get_paths(dir_path);
    rsx! {
        div {
            style: "display: flex; flex-direction: column;",

            if level == 0 {
                div { style: "width: 100%; display: flex; background-color: #222; padding: 6px 6px; font-size: 14px; border-bottom: 2px solid #333", 
                    div { style: "display: inline-flex; width: 60%;", "Name" }
                    div { style: "display: inline-flex; width: 20%", "Type" }
                    div { style: "display: inline-flex; width; 20%", "Last Modified" }
                }
            }
            for path in paths {
                if path.path().is_dir() {
                    ListFolder { path: path.path(), level: level }
                } else {
                    ListFile { path: path.path(), level: level }
                }
            }
        }
    }
}

#[component]
fn ListFile(path: PathBuf, level: usize) -> Element { 
    let mut focused = use_signal(|| false);

    let metadata = fs::metadata(&path)?;
    let mut date = "--".to_string();

    if let Ok(modified) = metadata.modified() {
        let datetime: DateTime<Utc> = modified.into();
        date = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    }

    let background_color = use_memo(move || { if focused() {"#334"} else {"#333"}});

    rsx! {
        div {
            onclick: move |_| {
                focused.toggle();
            },
            style: "border-bottom: 4px solid #222; border-top: 2px solid #111; padding: 4px 4px 4px {level * 22 + 10}px; background-color: {background_color};",

            div { style: "font-size: 14px", 
                        div { style: "display: inline-flex; width: 60%", 
                            img { src: IMG_FILE, style: "width: 16px; height: 18px; padding-right: 8px;"}
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
fn ListFolder(path: PathBuf, level: usize) -> Element {
    let mut is_expanded = use_signal(|| false);
    let mut focused = use_signal(|| false);
    let metadata = fs::metadata(&path)?;
    let mut date = "--".to_string();

    if let Ok(modified) = metadata.modified() {
        let datetime: DateTime<Utc> = modified.into();
        date = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    }

    let background_color = use_memo(move || { if focused() {"#334"} else {"#333"}});

    rsx! {
        div {
            onclick: move |_| { 
                is_expanded.toggle();
                focused.toggle();
             },
            style: "border-bottom: 4px solid #222; border-top: 2px solid #111; padding: 4px 4px 4px {level * 22 + 10}px; background-color: {background_color};",

            div { style: "font-size: 14px;", 
                        div { style: "display: inline-flex; width: 60%;", 
                            if is_expanded()
                                { div { style: "color: turquoise; margin-right: 8px;", " ↓ "} }
                            else
                                { div { style: "color: turquoise; margin-right: 8px;", " → "} }
                            img { src: IMG_FOLDER, style: "width: 16px; height: 18px; padding-right: 8px;"}
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