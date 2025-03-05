use dioxus::prelude::*;

use crate::utils::ModeContext;

use crate::components::FileExplorer;

#[component]
pub fn FolderStructure(dir_entry: String, level: usize) -> Element {

    let mut is_expanded = use_signal(|| false);
    rsx! {

        if *use_context::<ModeContext>().mode.read() {

        } else {
            div {
                class: "directory",
                style: "padding-left: {level * 20}px;", // Indent based on level
                button { 
                    onclick: move |_| { is_expanded.toggle(); }, 
                    "Folder: {dir_entry:?}" 
                }
            }
            if *is_expanded.read() {
                FileExplorer { dir_path: dir_entry, level: level + 1 } // Recursive call on FileExplorer
            }
            br {  }
        }
    }
}