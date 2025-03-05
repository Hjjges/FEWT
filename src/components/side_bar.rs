use dioxus::prelude::*;
use std::path::PathBuf;
use std::fs;


use crate::utils::DirectoryContext;
use crate::utils::ModeContext;

#[component]
pub fn SideBar() -> Element {
    let quick_access: Vec<PathBuf> = vec![
        PathBuf::from("/Users/hgregory/Documents"),
        PathBuf::from("/Users/hgregory/Downloads"),
        PathBuf::from("/Users/hgregory/TestStuff"),
    ];
    
    let hgregory = PathBuf::from("/Users/hgregory");
    let home_dirs = fs::read_dir(&hgregory)
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect::<Vec<PathBuf>>();

    rsx! {
        div { class: "side-bar",
            h3 { "Quick Access" }    
            for folder in quick_access.clone() {
                div { class: "button button-secondary", onclick: move |_| { consume_context::<DirectoryContext>().current_directory.set(folder.clone()); },
                    "{folder.file_name().unwrap().to_string_lossy()}"
                }
            }

            div {
                style: "border: white 1px solid; margin-top: 20px"
            }

            h3 { "Functionality" }   
            button {
                class: "button button-primary",
                onclick: move |_| {
                    let dir = use_context::<DirectoryContext>().current_directory.read().clone();
                    if let Some(parent) = dir.parent() {
                        consume_context::<DirectoryContext>().current_directory.set(parent.to_path_buf());
                    }
                
                },
                "Go back"
            }
            button {
                class: "button button-primary",
                onclick: move |_| {
                    let test = *use_context::<ModeContext>().mode.read();
                    consume_context::<ModeContext>().mode.set(!test)
                },
                div { "Icon Mode: " span { style: "color: yellow", "{use_context::<ModeContext>().mode.read()}" }}
            }

            div {
                style: "border: white 1px solid; margin-top: 20px"
            }

            h3 { "Home" }
            for folder in home_dirs.clone() {
                div { class: "button button-secondary", onclick: move |_| { consume_context::<DirectoryContext>().current_directory.set(folder.clone()); },
                    "{folder.file_name().unwrap().to_string_lossy()}"
                }
            }
        }
        div { style: "margin-left: 260px; padding: 20px;" } // TODO: clean up this weird css formatting
    }
}