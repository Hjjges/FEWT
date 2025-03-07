use dioxus::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;

use crate::utils::{DirectoryContext, DirectoryHistory, ModeContext};

const BACK: Asset = asset!("/assets/media-step-back-arrow-svgrepo-com.svg");
const FORWARD: Asset = asset!("/assets/forward-media-step-svgrepo-com.svg");

#[component]
pub fn TopBar() -> Element {
    let dir = Rc::new(use_context::<DirectoryContext>().current_directory.read().to_string());
    rsx! {
        div { class: "top-bar-content", 
            BackArrow { dir: dir.clone() }
            ForwardArrow { dir: dir.clone() }
            div { "{dir}" } // Show current directory
            ChangeMode { }
        }
    }
}

#[component]
fn BackArrow(dir: String) -> Element {
    rsx! {
        img { 
            src: BACK, 
            class: "arrows-path-navigation icon", 
            onclick: move |_| {
                if let Some(parent) = PathBuf::from(&dir).parent() {
                    let str = parent.to_string_lossy().to_string();
                    consume_context::<DirectoryContext>().current_directory.set(str.clone());
                    consume_context::<DirectoryHistory>().directory_history.write().push(dir.clone())
                } else {
                    println!("No back directories remaining in stack");
                }
            }, 
        }
    }
}

#[component]
fn ForwardArrow(dir: String) -> Element {
    rsx! {
        img { 
            src: FORWARD, 
            class: "arrows-path-navigation icon", 
            onclick: move |_| {
                // i guess you should also entirely remove any directories in the stack, if the new path goes into a new directory structure.
                // implement this whenever ? doesn't matter too much
                let history = consume_context::<DirectoryHistory>().directory_history.write().pop();
                if let Some(popped_dir) = history {
                    consume_context::<DirectoryContext>().current_directory.set(popped_dir);
                } else {
                    println!("No forward directories left in stack");
                }
            }, 
        }
    }
}

#[component]
fn ChangeMode() -> Element {
    rsx! {
        div { 
            style: "cursor: pointer; position: absolute; top: 50%; left: 95%; transform: translate(-50%, -50%);", 
            class: "icon", 
            onclick: move |_| {
                let read_mode = *use_context::<ModeContext>().mode.read();
                consume_context::<ModeContext>().mode.set(!read_mode)
            },
            "[=]" 
        }
    }
}