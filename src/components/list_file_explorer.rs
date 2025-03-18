use dioxus::prelude::*;

use crate::COPIED_PATH;
use crate::utils::{DioxusContextMenu, detect_extension, FileEntry};
use crate::helpers::get_entries;
use crate::components::FileExplorer;

const IMG_FILE: Asset = asset!("/assets/icons8-file.svg");
const IMG_FOLDER: Asset = asset!("/assets/folder-svgrepo-com.svg");

#[component]
pub fn ListFileExplorer(dir_path: String, level: usize) -> Element {
    let entries = get_entries(dir_path).unwrap(); // handle this error later
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
            for entry in entries {
                if entry.is_dir {
                    ListFolder { entry, level: level }
                } else {
                    ListFile { entry, level: level }
                }
            }
        }
    }
}

#[component]
fn ListFile(entry: FileEntry, level: usize) -> Element { 
    let mut focused = use_signal(|| false);
    let background_color = use_memo(move || { if focused() {"#334"} else {"#181818"}});

    rsx! {
        div {
            onclick: move |_| {
                focused.toggle();
            },
            class: "list-file",
            style: "padding: 4px 4px 4px {level * 22 + 10}px; background-color: {background_color};",

            div { style: "font-size: 14px", 
                oncontextmenu: move |e| { e.prevent_default(); 
                    *COPIED_PATH.write() = entry.path_string.clone();
                    DioxusContextMenu::default();
                },
                div { style: "display: inline-flex; width: 60%", 
                    img { src: IMG_FILE, style: "width: 16px; height: 18px; padding-right: 8px;"}
                    "{entry.name}" 
                }
                if let Some(ext) = entry.path.extension() {
                    div { style: "display: inline-flex; width: 20%", "{detect_extension(ext.to_string_lossy().to_string())} " }
                } else {
                    div { style: "display: inline-flex; width: 20%", "File" }
                }
                div { style: "display: inline-flex; width; 20%", "{entry.modified}" }
            }
        }
    }
}

#[component]
fn ListFolder(entry: FileEntry, level: usize) -> Element {
    let mut is_expanded = use_signal(|| false);
    let mut focused = use_signal(|| false);
    let background_color = use_memo(move || { if focused() {"#334"} else {"#181818"}});

    rsx! {
        div {
            onclick: move |_| { 
                is_expanded.toggle();
                focused.toggle();
             },
            class: "list-folder",
            style: "padding: 4px 4px 4px {level * 22 + 10}px; background-color: {background_color};",

            div { style: "font-size: 14px;", 
                div { style: "display: inline-flex; width: 60%;", 
                    oncontextmenu: move |e| { e.prevent_default(); 
                        *COPIED_PATH.write() = entry.path_string.clone();
                        DioxusContextMenu::default();
                    },
                    if is_expanded()
                        { div { style: "color: turquoise; margin-right: 8px;", " ↓ "} }
                    else
                        { div { style: "color: turquoise; margin-right: 8px;", " → "} }
                    img { src: IMG_FOLDER, style: "width: 16px; height: 18px; padding-right: 8px;"}
                    "{entry.name}" 
                }
                div { style: "display: inline-flex; width: 20%", "Folder" }
                div { style: "display: inline-flex; width; 20%", "{entry.modified}" }
            }
        }
        if *is_expanded.read() {
            FileExplorer { dir_path: entry.path.to_string_lossy(), level: level + 1 } // Recursive call on FileExplorer
        }
    }
}