use dioxus::prelude::*;
use std::path::PathBuf;

#[derive(Clone, Copy)]
pub struct DirectoryContext {
    pub current_directory: Signal<PathBuf>,
}