extern crate directories;

use directories::{UserDirs, BaseDirs, ProjectDirs};
use dioxus::prelude::*;
use std::path::PathBuf;
use crate::utils::DirectoryContext;
use crate::components::ResizerBar;


#[component]
pub fn SideBar() -> Element {
    let mut quick_access = Vec::new();
    let mut favourites: Vec<PathBuf> = Vec::new(); // implement toml config

    if let Some(user_dirs) = UserDirs::new() {
        let test = [
            user_dirs.desktop_dir(),
            user_dirs.document_dir(),
            user_dirs.download_dir(),
            user_dirs.video_dir(),
        ];
    
        quick_access.extend(test.iter().flatten().map(|p| p.to_path_buf()));
    }
    
    
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
        div { class: "side-bar-entry", span {"{title} +"} }
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
