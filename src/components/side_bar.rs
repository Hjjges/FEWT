use dioxus::prelude::*;
use std::path::PathBuf;

use crate::utils::DirectoryContext;

#[component]
pub fn SideBar() -> Element {
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
        div { class: "side-bar-content",
            QuickAccess { quick_access: quick_access.clone() }
            Divider { }
            Favourites { favourites: favourites.clone() }
        }
    }
}


#[component]
fn Divider() -> Element {
    rsx! {
        div { class: "divider" }
    }
}

#[component]
fn QuickAccess(quick_access: Vec<PathBuf>) -> Element {
    rsx! {
        div { class: "side-bar-entry", "Quick Access" }    
            for folder in quick_access.clone() {
                div { class: "button button-secondary", onclick: move |_| { consume_context::<DirectoryContext>().current_directory.set(folder.to_string_lossy().to_string()); },
                    // add img here , cute icon?
                    "{folder.file_name().unwrap().to_string_lossy().to_string()}"
                }
            }
    }
}

#[component]
fn Favourites(favourites: Vec<PathBuf>) -> Element {
    rsx! {
        div { class: "side-bar-entry", "Favourites" }    
        for folder in favourites {
            div { class: "button button-secondary", onclick: move |_| { consume_context::<DirectoryContext>().current_directory.set(folder.file_name().unwrap().to_string_lossy().to_string()); },
                "{folder.file_name().unwrap().to_string_lossy().to_string()}"
            }
        }
    }
}