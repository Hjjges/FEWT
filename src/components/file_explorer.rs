use dioxus::prelude::*;
use crate::utils::ModeContext;
use crate::components::{IconFileExplorer, ListFileExplorer};

// We call dir_path as a property instead of using global context, because file explorer is used recursively in list mode.
#[component]
pub fn FileExplorer(dir_path: String, level: usize) -> Element {
    rsx! {
        div { class: "file-explorer",
            if *use_context::<ModeContext>().mode.read() 
                { IconFileExplorer { dir_path: dir_path } } 
            else 
                { ListFileExplorer { dir_path: dir_path, level: level } }
        }
    }
}