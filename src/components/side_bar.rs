use dioxus::prelude::*;
use std::path::PathBuf;

use crate::utils::DirectoryContext;
use crate::components::ResizerBar;


#[component]
pub fn SideBar() -> Element {

    // Consider using SQLite file database for storing this information
    let quick_access: Vec<PathBuf> = vec![
        PathBuf::from("/Users/hgregory/Documents"),
        PathBuf::from("/Users/hgregory/Downloads"),
        PathBuf::from("/Users/hgregory/Desktop"),
    ];

    let favourites: Vec<PathBuf> = vec![
        PathBuf::from("/Users/hgregory/TestStuff"),
        PathBuf::from("/Users/hgregory/FeralTools"),
        PathBuf::from("/Users/hgregory/dumps"),
        PathBuf::from("/Users/hgregory/Scripts")
    ];
    
    rsx! {
        div { class: "side-bar",
            div { class: "side-bar-container",
            div { style: "grid-column: 1; border-right: 1px solid #333;",
                SideBarEntry { entries: quick_access.clone(), title: "QUICK ACCESS" }
                Divider { }
                SideBarEntry { entries: favourites.clone(), title: "FAVOURITES" }
            }
            ResizerBar { class: "resizer-side", custom_style: "grid-column: 2;"} }
        }
    }
}

#[component]
fn SideBarEntry(entries: Vec<PathBuf>, title: String) -> Element {
    rsx! {
        div { class: "side-bar-entry", "{title}" }
        for item in entries.clone() {
            div { class: "button button-secondary",
                onclick: move |_| {
                    consume_context::<DirectoryContext>().current_directory.set(item.to_string_lossy().to_string());
                },
                div { "{item.file_name().unwrap().to_string_lossy().to_string()}" }
            }
        }
    }

}

#[component]
fn Divider() -> Element {
    rsx! { div { class: "divider" } }
}
