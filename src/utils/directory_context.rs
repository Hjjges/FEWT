use dioxus::prelude::*;
#[derive(Clone, Copy)]
pub struct DirectoryContext {
    pub current_directory: Signal<String>,
}