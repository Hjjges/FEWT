use dioxus::prelude::*;
use std::path::PathBuf;

const IMG_FILE: Asset = asset!("/assets/icons8-file.svg");

#[component]
pub fn FileIcon(path: String) -> Element {
    let file_name = PathBuf::from(&path).file_name().unwrap().to_string_lossy().to_string();
    rsx! {
        div { class: "file",
            img { src: IMG_FILE, class: "file-img" }
            p { "{file_name}" }
        }
    }
}