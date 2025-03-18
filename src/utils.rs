mod extension_detection;
mod dioxus_bridge;
mod shell_context;
mod app_config;
mod context_menu;
mod file_entry;

pub use shell_context::ShellContext;
pub use dioxus_bridge::{initialize_bash, initialize_dioxus_bridge};
pub use app_config::{AppConfig, Genre, Mode};
pub use context_menu::DioxusContextMenu;
pub use file_entry::FileEntry;