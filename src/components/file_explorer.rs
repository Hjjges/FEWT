use dioxus::prelude::*;
use crate::components::{ColumnFileExplorer, IconFileExplorer, ListFileExplorer};
use crate::utils::Mode;

// We call dir_path as a property instead of using global context, because file explorer is used recursively in list mode.
#[component]
pub fn FileExplorer(dir_path: String, level: usize) -> Element {
    let mode = use_context::<Signal<Mode>>()();
    let content = match mode {
        Mode::Icon => rsx!(IconFileExplorer { dir_path }),
        Mode::List => rsx!(ListFileExplorer { dir_path, level }),
        Mode::Column => rsx!(ColumnFileExplorer { dir_path, level })
    };
    rsx! {
        div { class: "file-explorer",
            {content}
        }
    }
}