use dioxus::prelude::*;
use crate::CURRENT_MODE;
use crate::components::{IconFileExplorer, ListFileExplorer};

// We call dir_path as a property instead of using global context, because file explorer is used recursively in list mode.
#[component]
pub fn FileExplorer(dir_path: String, level: usize) -> Element {
    rsx! {
        div { class: "file-explorer",
            if CURRENT_MODE()
                { IconFileExplorer { dir_path: dir_path } } 
            else 
                { ListFileExplorer { dir_path: dir_path, level: level } }
        }
    }
}