use dioxus::prelude::*;
use dioxus::logger::tracing;
use std::path::PathBuf;
use std::fs;

use crate::COPIED_PATH;
use crate::utils::{DioxusContextMenu, FileEntry};
use crate::helpers::get_entries;
use crate::components::FileExplorer;

const IMG_FILE: Asset = asset!("/assets/icons8-file.svg");
const IMG_FOLDER: Asset = asset!("/assets/folder-svgrepo-com.svg");

// Clicking back should remove the latest column in the vec?

#[component]
pub fn ColumnFileExplorer(dir_path: String, level: usize) -> Element {
    let mut columns = use_signal(|| vec![ dir_path.clone() ]);
    rsx! {
        div { style: "display: grid; grid-auto-flow: column; grid-template-columns: repeat(auto-fit, 300px); overflow-x: auto;",
            for (index, column) in columns().iter().enumerate() {
                Column { dir_path: column, level: index, columns }
            }
        }
    }
}

#[component]
fn Column(dir_path: String, level: usize, columns: Signal<Vec<String>>) -> Element {
    let entries = get_entries(dir_path).unwrap();
    rsx! {
        div { style: "width: 300px; box-sizing: border-box; flex-shrink: 0; overflow-y: scroll; border-right: 1px solid #333;", 
            div { style: "display: flex; flex-direction: column;",
                for entry in entries.clone() {
                    if entry.is_dir {
                        ColumnFolder { entry, level, columns }
                    } else {
                        ColumnFile { entry, level }
                    }
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
fn ColumnFolder(entry: FileEntry, level: usize, columns: Signal<Vec<String>>) -> Element {
    let mut is_expanded = use_signal(|| false);
    let mut focused = use_signal(|| false);
    let copy = entry.path_string.clone();
    let background_color = use_memo(move || { if focused() {"#334"} else {"#181818"}});

    let mut vec_columns = columns().clone();

    use_effect(move || {
        // If it is expanded and the current columns don't contain the path, add it to the column signal vector
        if *is_expanded.read() && !columns.write().contains(&entry.path_string){
            columns.write().push(entry.path_string.clone());

        // If it is not expanded (e.g. you just clicked to turn it off), find the existing index of that column entry and remove everything after it
        } else if !*is_expanded.read() {
            tracing::info!("value of entry: {:?}", entry.path_string.clone());
            tracing::info!("before: {:?}", vec_columns);
            while let Some(popped) = vec_columns.pop() {
                if popped == entry.path_string.clone() {
                    break;
                }
            }
            tracing::info!("after: {:?}", vec_columns);
            // copy resulting vec_columns into the column signal vector - if length is 0, means we didn't find a match, meaning they tried to open new folders.
            // consider doing this check in the beginning by doing vec_columns.contains(&entry.path_string) ? break. - it will avoid unnecessary pops and computation
            if vec_columns.len() != 0 {
                *columns.write() = vec_columns.clone();
            }

            // additionally, its possible to open multiple layers even though theyre at the same level. Check if the index after the current one is empty, if its
            // empty, then its ok to add. Otherwise, its not ok, remove the existing index and then replace it with the new one.
        }
    });

    rsx! {
        div {
            onclick: move |_| {
                is_expanded.toggle();
                focused.toggle();
             },
             oncontextmenu: move |e| { e.prevent_default();
                *COPIED_PATH.write() = copy.clone();
                DioxusContextMenu::default();
            },
            class: "list-folder",
            style: "padding: 4px 4px 4px 10px; background-color: {background_color};",

            div { style: "font-size: 14px; display: inline-flex; width: 100%;",
                img { src: IMG_FOLDER, style: "width: 16px; height: 18px; padding-right: 8px;"}
                "{entry.name}"
                if is_expanded()
                    { div { style: "color: turquoise; margin-left: 10px;", " → "} }
            }
            
        }
    }
}