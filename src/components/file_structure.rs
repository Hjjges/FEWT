use dioxus::prelude::*;

#[component]
pub fn FileStructure(file_entry: String, level: usize ) -> Element {

    rsx! {
        div { 
            class: "file",
            style: "padding-left: {level * 20}px;", // Indent based on level
            button { 
                "File: {file_entry:?}"
            }
        }
        br {  }
    }
}