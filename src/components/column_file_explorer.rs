use dioxus::prelude::*;

use crate::COPIED_PATH;
use crate::utils::{DioxusContextMenu, FileEntry};
use crate::helpers::get_entries;
use crate::components::FileExplorer;

const IMG_FILE: Asset = asset!("/assets/icons8-file.svg");
const IMG_FOLDER: Asset = asset!("/assets/folder-svgrepo-com.svg");

#[component]
pub fn ColumnFileExplorer(dir_path: String, level: usize) -> Element {
    let entries = get_entries(dir_path).unwrap();
    rsx! {
        div {
            style: "display: flex; flex-direction: column;",

            // Based on the level we can create the next column.
            for entry in entries {
                if entry.is_dir {
                    ColumnFolder { entry, level: level }
                } else {
                    ColumnFile { entry, level: level }
                }
            }
        }
    }
}

#[component]
fn ColumnFile(entry: FileEntry, level: usize) -> Element {
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
            style: "padding: 4px 4px 4px 10px; background-color: {background_color};",

            div { style: "font-size: 14px; display: inline-flex; width: 100%",
                img { src: IMG_FILE, style: "width: 16px; height: 18px; padding-right: 8px;"}
                "{entry.name}" 
            }
        }
    }
}

#[component]
fn ColumnFolder(entry: FileEntry, level: usize) -> Element {
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
            style: "padding: 4px 4px 4px 10px; background-color: {background_color};",

            div { style: "font-size: 14px; display: inline-flex; width: 100%;",
                if is_expanded()
                    { div { style: "color: turquoise; margin-right: 8px;", " → "} }
                img { src: IMG_FOLDER, style: "width: 16px; height: 18px; padding-right: 8px;"}
                "{entry.name}"
            }
            
        }
        if *is_expanded.read() {
            FileExplorer { dir_path: entry.path.to_string_lossy(), level: level + 1 } // Recursive call on FileExplorer
        }
    }
}