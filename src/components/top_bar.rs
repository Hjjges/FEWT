use dioxus::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;

use crate::utils::{DirectoryContext, DirectoryHistory, ModeContext};

#[component]
pub fn TopBar() -> Element {
    let dir = Rc::new(use_context::<DirectoryContext>().current_directory.read().to_string());
    rsx! {
        div { class: "top-bar",
            div { class: "top-bar-content", 
                // new custom top
                div { style: "display: flex; font-size: 18px; color: turquoise; padding: 4px; margin: 4px; border: 1px solid turquoise", 
                    BackArrow { dir: dir.clone() }
                    ForwardArrow { dir: dir.clone() }
                    div { style: "margin: 0 8px", "{dir}" }
                }
            }
            ChangeMode { }
        }
    }
}

#[component]
fn BackArrow(dir: String) -> Element {
    rsx! {
        div { class: "arrows-path-navigation icon",
            onclick: move |_| {
                if let Some(parent) = PathBuf::from(&dir).parent() {
                    let str = parent.to_string_lossy().to_string();
                    consume_context::<DirectoryContext>().current_directory.set(str.clone());
                    consume_context::<DirectoryHistory>().directory_history.write().push(dir.clone())
                } else {
                    println!("No back directories remaining in stack");
                }
            }, 
            "<",
        }
    }
}

#[component]
fn ForwardArrow(dir: String) -> Element {
    rsx! {
        div { style: "margin-left: 8px", class: "arrows-path-navigation icon",
            onclick: move |_| {
                // if the new path goes into a new directory structure, remove any directories in the stack that were in a old route.
                let history = consume_context::<DirectoryHistory>().directory_history.write().pop();
                if let Some(popped_dir) = history {
                    consume_context::<DirectoryContext>().current_directory.set(popped_dir);
                } else {
                    println!("No forward directories left in stack");
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
            onclick: move |_| {
                let read_mode = *use_context::<ModeContext>().mode.read();
                consume_context::<ModeContext>().mode.set(!read_mode)
            },
            "[=]" 
        }
    }
}