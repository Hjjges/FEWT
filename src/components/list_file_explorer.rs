use dioxus::prelude::*;

use crate::{SORT, COPIED_PATH};
use crate::utils::{DioxusContextMenu, detect_extension, FileEntry};
use crate::helpers::get_entries;
use crate::components::FileExplorer;

const IMG_FILE: Asset = asset!("/assets/icons8-file.svg");
const IMG_FOLDER: Asset = asset!("/assets/folder-svgrepo-com.svg");

#[component]
pub fn ListFileExplorer(dir_path: String, level: usize) -> Element {
    let sort_down = use_signal(|| false);
    let entries = get_entries(dir_path).unwrap(); // handle this error later
    rsx! {
        div {
            style: "display: flex; flex-direction: column;",

            if level == 0 {
                div { style: "width: 100%; display: flex; background-color: #222; padding: 6px 6px; font-size: 14px; border-bottom: 2px solid #333", 
                    Headers { header_type: "name", desc: "Name", width: "60%", sort_down}
                    Headers { header_type: "extension", desc: "Type", width: "20%", sort_down }
                    Headers { header_type: "modified", desc: "Last Modified", width: "20%", sort_down }
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
// ↑
#[component]
fn Headers(header_type: String, desc: String, width: String, sort_down: Signal<bool>) -> Element {
    let header = header_type.clone();
    let header2 = header.clone();
    let sort_arrow = use_memo(move || {
        if SORT().trim_start_matches('-') == header {
            if sort_down() { "↓" }
            else { "↑" }
        } else { "" }
    });
    let color = use_memo(move || if SORT().trim_start_matches('-') == header2 { "turquoise" } else { "white" });
    rsx! {
        div { style: "display: inline-flex; width: {width}; cursor: pointer; color: {color}",
            onclick: move |_| {
                if sort_down() {
                    *SORT.write() = header_type.clone()
                } else {
                    *SORT.write() = format!("-{}", header_type.clone());
                }
                *sort_down.write() = !sort_down();
            },
            "{sort_arrow} {desc}"
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
            oncontextmenu: move |e| { e.prevent_default();
                *COPIED_PATH.write() = entry.path_string.clone();
                DioxusContextMenu::default();
            },
            class: "list-file",
            style: "padding: 4px 4px 4px {level * 22 + 10}px; background-color: {background_color};",

            div { style: "font-size: 14px",
                div { style: "display: inline-flex; width: 60%",
                    img { src: IMG_FILE, style: "width: 16px; height: 18px; padding-right: 8px;"}
                    "{entry.name}" 
                }
                div { style: "display: inline-flex; width: 20%", "{entry.extension}" }
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
             oncontextmenu: move |e| { e.prevent_default();
                *COPIED_PATH.write() = entry.path_string.clone();
                DioxusContextMenu::default();
            },
            class: "list-folder",
            style: "padding: 4px 4px 4px {level * 22 + 10}px; background-color: {background_color};",

            div { style: "font-size: 14px;", 
                div { style: "display: inline-flex; width: 60%;",
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