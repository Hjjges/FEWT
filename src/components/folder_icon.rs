use dioxus::prelude::*;
use std::path::PathBuf;

use crate::utils::DirectoryContext;

const IMG_FOLDER: Asset = asset!("/assets/folder-svgrepo-com.svg");


#[component]
pub fn FolderIcon(path: PathBuf) -> Element {
    rsx! {
        div { class: "folder", onclick: move |_| { consume_context::<DirectoryContext>().current_directory.set(path.clone()); }, 
            img { src: IMG_FOLDER, class: "folder-img" }
            p { "{path.file_name().unwrap().to_string_lossy()}" }
        }
    }
}