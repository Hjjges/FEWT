use dioxus::prelude::*;
use std::path::PathBuf;

const IMG_FILE: Asset = asset!("/assets/icons8-file.svg");

#[component]
pub fn FileIcon(path: PathBuf) -> Element {
    rsx! {
        div { class: "file",
            img { src: IMG_FILE, class: "file-img" }
            p { "{path.file_name().unwrap().to_string_lossy()}" }
        }
    }
}