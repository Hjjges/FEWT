use dioxus::prelude::*;
use dioxus::logger::tracing;
use std::path::PathBuf;

use crate::{CURRENT_DIR, CURRENT_MODE, DIR_HISTORY};

#[component]
pub fn TopBar() -> Element {
    rsx! {
        div { class: "top-bar",
            div { class: "top-bar-content", 
                // new custom top
                div { style: "display: flex; font-size: 18px; color: turquoise; padding: 4px; margin: 4px; border: 1px solid turquoise", 
                    BackArrow { }
                    ForwardArrow { }
                    div { style: "margin: 0 8px", "{CURRENT_DIR}" }
                }
            }
            ChangeMode { }
        }
    }
}

#[component]
fn BackArrow() -> Element {

    rsx! {
        div { class: "arrows-path-navigation icon",
            onclick: move |_| {
                let path = PathBuf::from(&CURRENT_DIR());
                if let Some(parent) = path.parent() {
                    let str = parent.to_string_lossy().to_string();
                    *CURRENT_DIR.write() = str.clone();
                    DIR_HISTORY.write().push(CURRENT_DIR());
                } else {
                    tracing::warn!("No back directories remaining in stack");
                }
            }, 
            "<",
        }
    }
}

#[component]
fn ForwardArrow() -> Element {
    rsx! {
        div { style: "margin-left: 8px", class: "arrows-path-navigation icon",
            onclick: move |_| {
                // if the new path goes into a new directory structure, remove any directories in the stack that were in a old route.
                let history = DIR_HISTORY.write().pop();
                if let Some(popped_dir) = history {
                    *CURRENT_DIR.write() = popped_dir;
                } else {
                    tracing::warn!("No forward directories left in stack");
                }
            }, 
            ">" 
        }
    }
}

#[component]
fn ChangeMode() -> Element {
    rsx! {
        div { 
            style: "display: flex; justify-content: center; align-items: center; cursor: pointer;", 
            class: "icon", 
            onclick: move |_| { *CURRENT_MODE.write() = !CURRENT_MODE() },
            "[=]" 
        }
    }
}