use dioxus::prelude::*;
use std::path::PathBuf;

use crate::utils::{DirectoryContext, DirectoryHistory};

const IMG_FOLDER: Asset = asset!("/assets/folder-svgrepo-com.svg");


#[component]
pub fn FolderIcon(path: String) -> Element {
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