use dioxus::prelude::*;
use std::path::PathBuf;
use crate::utils::{AppConfig, Genre, DioxusContextMenu};
use crate::{CURRENT_DIR, COPIED_PATH};

use crate::components::ResizerBar;

#[component]
pub fn SideBar() -> Element {
    let app_config = try_use_context::<Signal<AppConfig>>().expect("Could not find AppConfig context");
    rsx! {
        div { class: "side-bar",
            div { class: "side-bar-container",
            div { style: "grid-column: 1; border-right: 1px solid #333;",
                SideBarEntry { entries: app_config().quick_access.clone(), title: "QUICK ACCESS", classes: "button button-secondary", category: Genre::QuickAccess }
                Divider { }
                SideBarEntry { entries: app_config().favourites.clone(), title: "FAVOURITES", classes: "button button-favourites", category: Genre::Favourites }
                //div { style: "margin-bottom: 24px;"}
            }
            ResizerBar { class: "resizer-side", custom_style: "grid-column: 2;"} }
        }
    }
}

#[component]
fn SideBarEntry(entries: Vec<PathBuf>, title: String, classes: String, category: Genre) -> Element {
    rsx! {
        div { class: "side-bar-entry", span {"{title}"} }
        for item in entries.clone() {
            SideBarEntryComponent { item: item, classes: &classes, category: category }
        }
    }
}

#[component]
fn SideBarEntryComponent(item: PathBuf, classes: String, category: Genre) -> Element {
    let item_cloned = item.clone();
    rsx! {
        div { class: "{classes}",
                oncontextmenu: move |e| { e.prevent_default(); 
                    *COPIED_PATH.write() = item.to_string_lossy().to_owned().to_string();
                    DioxusContextMenu::side_bar(category);
                },
                onclick: move |_| { *CURRENT_DIR.write() = item_cloned.to_string_lossy().to_string() },
                div { "{item.file_name().unwrap().to_string_lossy().to_string()}" }
        }
    }
}

#[component]
fn Divider() -> Element {
    rsx! { div { class: "divider" } }
}
