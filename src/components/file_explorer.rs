use dioxus::prelude::*;
use std::fs;

use crate::utils::DirectoryContext;
use crate::utils::ModeContext;

use crate::components::FolderIcon;
use crate::components::FileIcon;

use crate::components::FolderStructure;
use crate::components::FileStructure;

#[component]
pub fn FileExplorer(dir_path: String, level: usize) -> Element {
    // Setup
    let paths = fs::read_dir(&dir_path)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    // Render the directories
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px; padding: 10px;",
            div { class: "path-tracker", "{use_context::<DirectoryContext>().current_directory.read().clone().to_string_lossy()}" },
            div {
                style: "display: inline-flex; flex-wrap: wrap; gap: 50px; margin-top: 40px",
                for path in paths {
                    if *use_context::<ModeContext>().mode.read() {
                        if path.path().is_dir() {
                            FolderIcon { path: path.path() }
                        } else {
                            FileIcon { path: path.path() }
                        }
                    }
                    else {
                        if path.path().is_dir() {
                            button {
                                onclick: move |_| {
                                    let dir = use_context::<DirectoryContext>().current_directory.read().clone();
                                    if let Some(parent) = dir.parent() {
                                        consume_context::<DirectoryContext>().current_directory.set(parent.to_path_buf());
                                    }
                                },
                                "Make Main Folder"
                            }
                            FolderStructure { dir_entry: path.path().to_string_lossy(), level: level }
                        } else {
                            FileStructure { file_entry: path.path().to_string_lossy(), level: level }
                        }
                    }
                }
            }
        }
        
    }
}
