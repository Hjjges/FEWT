use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct DirectoryHistory {
    pub directory_history: Signal<Vec<String>>,
}