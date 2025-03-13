use dioxus::prelude::*;
use std::path::PathBuf;

use crate::utils::{DirectoryContext, DirectoryHistory};
use crate::helpers::get_paths;

const IMG_FILE: Asset = asset!("/assets/icons8-file.svg");
const IMG_FOLDER: Asset = asset!("/assets/folder-svgrepo-com.svg");

#[component]
pub fn IconFileExplorer(dir_path: String) -> Element {
    let paths = get_paths(dir_path);
    rsx! {
        div {
            style: "display: inline-flex; flex-wrap: wrap; gap: 10%",
            for path in paths {
                if path.path().is_dir() {
                    IconFolder { path: path.path().to_string_lossy().to_string() }
                } else {
                    IconFile { path: path.path().to_string_lossy().to_string() }
                }
            }
        }
    }
}

#[component]
fn IconFile(path: String) -> Element {
    let file_name = PathBuf::from(&path).file_name().unwrap().to_string_lossy().to_string();
    rsx! {
        div { class: "file",
            img { src: IMG_FILE, class: "file-img" }
            p { "{file_name}" }
        }
    }
}

#[component]
fn IconFolder(path: String) -> Element {
    let file_name = PathBuf::from(&path).file_name().unwrap().to_string_lossy().to_string();
    rsx! {
        div { 
            class: "folder", 
            onclick: move |_| { 
                consume_context::<DirectoryContext>().current_directory.set(path.clone());
                let history = use_context::<DirectoryHistory>().directory_history.read().clone();
                if !history.contains(&path) {
                    consume_context::<DirectoryHistory>().directory_history.write().push(path.clone())
                }
                
            }, 
            img { src: IMG_FOLDER, class: "folder-img" }
            p { "{file_name}" }
        }
    }
}