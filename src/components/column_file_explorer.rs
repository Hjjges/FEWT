use dioxus::prelude::*;
use dioxus::logger::tracing;
use std::fs;
use crate::{CURRENT_DIR, COPIED_PATH};
use crate::utils::{DioxusContextMenu, FileEntry};
use crate::helpers::get_entries;

const IMG_FILE: Asset = asset!("/assets/icons8-file.svg");
const IMG_FOLDER: Asset = asset!("/assets/folder-svgrepo-com.svg");

#[component]
pub fn ColumnFileExplorer() -> Element {
    let mut columns = use_signal(|| vec![ CURRENT_DIR() ]);

    use_effect(move || {
        if CURRENT_DIR() != *columns().get(0).unwrap() { *columns.write() = vec![ CURRENT_DIR() ]}
        tracing::info!("{:?}", columns());

    });
    rsx! { 
        div { style: "display: grid; grid-auto-flow: column; grid-template-columns: repeat(auto-fit, 300px); height: 100%; overflow-x: scroll;",
            for (index, column) in columns().iter().enumerate() {
                Column { dir_path: column, columns }
            }
        }
    }
}

#[component]
fn Column(dir_path: String, columns: Signal<Vec<String>>) -> Element {


    // we must check if dir_path is an actual dir or not.
    let metadata = fs::metadata(dir_path.clone()).unwrap();
    if metadata.is_dir() {
        let entries = get_entries(dir_path.clone()).unwrap();
        rsx! {
            div { style: "width: 300px; box-sizing: border-box; border-right: 1px solid #333; flex-shrink: 0; overflow: auto;", 
                div { style: "display: flex; flex-direction: column;",
                    for entry in entries.clone() {
                        if entry.is_dir {
                            ColumnFolder { entry, columns }
                        } else {
                            ColumnFile { entry, columns }
                        }
                    }
                }
            }
        }
    } else {
        let entry: FileEntry = FileEntry::new_file(dir_path.clone(), metadata).unwrap();
        let file_content = std::fs::read_to_string(entry.path_string).unwrap_or_else(|_| "Failed to load file".to_string());
        rsx! {
            div { style: "width: 80vh; box-sizing: border-box; border-right: 1px solid #333; flex-shrink: 0; overflow: auto;", 
                div { style: "width: 100%; height: 100%",
                    div { style: "border-radius: 4px; height: 40vh; border: 1px solid #333; padding: 16px 16px; margin: 16px 16px; overflow: auto;", pre { style: "white-space: pre-wrap; font-size: 14px;", {file_content} } }
                    div { style: "font-size: 16px; text-align: center", {entry.name} }
                }
            }
        }
    }
}

#[component]
fn ColumnFile(entry: FileEntry, columns: Signal<Vec<String>>) -> Element {
    let copy = entry.path_string.clone();
    let copy2 = entry.path_string.clone();
    let is_expanded: Memo<bool> = use_memo(move || columns().contains(&entry.path_string));
    let background_color = use_memo(move || { if is_expanded() {"#334"} else {"#181818"}});
    let mut vec_columns = columns().clone();


    rsx! {
        div {
            onclick: move |_| {
                let new_path = &copy2;
                if !*is_expanded.read() {
                    let parent_path = match new_path.rfind('/') {
                        Some(index) if index > 0 => &new_path[0..=index],
                        _ => "/",
                    };
                    let matching_index = vec_columns.iter().position(|path| path.starts_with(parent_path) && path != parent_path);
                    if let Some(index) = matching_index {
                        vec_columns.truncate(index);
                    }
                    vec_columns.push(new_path.clone());
                    *columns.write() = vec_columns.clone();
                } else if *is_expanded.read() && !columns().contains(&copy2) {
                    while let Some(popped) = vec_columns.pop() {
                        if popped == copy2 { break }
                    }
                    if vec_columns.len() != 0 {
                        *columns.write() = vec_columns.clone();
                    }
                }

             },
            oncontextmenu: move |e| { e.prevent_default();
                *COPIED_PATH.write() = copy.clone();
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
fn ColumnFolder(entry: FileEntry, columns: Signal<Vec<String>>) -> Element {
    let copy = entry.path_string.clone();
    let copy2 = entry.path_string.clone();
    let is_expanded: Memo<bool> = use_memo(move || columns().contains(&entry.path_string));
    let background_color = use_memo(move || { if is_expanded() {"#334"} else {"#181818"}});
    let mut vec_columns = columns().clone();


    rsx! {
        div {
            onclick: move |_| {
                let new_path = &copy2;
                if !*is_expanded.read() {
                    let parent_path = match new_path.rfind('/') {
                        Some(index) if index > 0 => &new_path[0..=index],
                        _ => "/",
                    };
                    let matching_index = vec_columns.iter().position(|path| path.starts_with(parent_path) && path != parent_path);
                    if let Some(index) = matching_index {
                        vec_columns.truncate(index);
                    }
                    vec_columns.push(new_path.clone());
                    *columns.write() = vec_columns.clone();
                } else if *is_expanded.read() && !columns().contains(&copy2) {
                    while let Some(popped) = vec_columns.pop() {
                        if popped == copy2 { break }
                    }
                    if vec_columns.len() != 0 {
                        *columns.write() = vec_columns.clone();
                    }
                }

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