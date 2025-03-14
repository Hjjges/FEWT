use dioxus::prelude::*;
use std::path::PathBuf;
use crate::utils::{AppConfig, DirectoryContext};
use crate::components::ResizerBar;

#[component]
pub fn SideBar() -> Element {
    rsx! {
        div { class: "side-bar",
            div { class: "side-bar-container",
            div { style: "grid-column: 1; border-right: 1px solid #333;",
                SideBarEntry { entries: use_context::<AppConfig>().quick_access, title: "QUICK ACCESS" }
                Divider { }
                SideBarEntry { entries: use_context::<AppConfig>().favourites, title: "FAVOURITES" }
            }
            ResizerBar { class: "resizer-side", custom_style: "grid-column: 2;"} }
        }
    }
}

#[component]
fn SideBarEntry(entries: Vec<PathBuf>, title: String) -> Element {
    rsx! {
        div { class: "side-bar-entry", span {"{title}"} }
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
