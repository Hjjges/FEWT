use dioxus::prelude::*;
use crate::{CURRENT_DIR, DIR_HISTORY, COPIED_PATH};

use crate::utils::{DioxusContextMenu, FileEntry};
use crate::helpers::get_entries;

const IMG_FILE: Asset = asset!("/assets/icons8-file.svg");
const IMG_FOLDER: Asset = asset!("/assets/folder-svgrepo-com.svg");

#[component]
pub fn IconFileExplorer() -> Element {
    let entries = get_entries(CURRENT_DIR()).unwrap();
    rsx! {
        div {
            style: "display: inline-flex; flex-wrap: wrap; gap: 10%",
            for entry in entries {
                if entry.is_dir {
                    IconFolder { entry }
                } else {
                    IconFile { entry }
                }
            }
        }
    }
}

#[component]
fn IconFile(entry: FileEntry) -> Element {
    rsx! {
        div { class: "file",
            oncontextmenu: move |e| { e.prevent_default(); 
                *COPIED_PATH.write() = entry.path_string.clone();
                DioxusContextMenu::default();
            },
            img { src: IMG_FILE, class: "file-img" }
            p { "{entry.name}" }
        }
    }
}

#[component]
fn IconFolder(entry: FileEntry) -> Element {
    let copied_entry = entry.clone();
    rsx! {
        div { 
            class: "folder", 
            oncontextmenu: move |e| { e.prevent_default(); 
                *COPIED_PATH.write() = entry.path_string.clone();
                DioxusContextMenu::default();
            },
            onclick: move |_| { 
                *CURRENT_DIR.write() = copied_entry.path_string.clone();
                if !DIR_HISTORY().clone().contains(&copied_entry.path_string) {
                    DIR_HISTORY.write().push(copied_entry.path_string.clone());
                }
                
            }, 
            img { src: IMG_FOLDER, class: "folder-img" }
            p { "{entry.name}" }
        }
    }
}