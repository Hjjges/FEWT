mod components;
mod utils;
mod helpers;

use dioxus::prelude::*;
use components::{TopBar, SideBar, FileExplorer, TerminalComponent, BottomBar};
use utils::{DirectoryContext, DirectoryHistory, ModeContext, initialize_bash, initialize_dioxus_bridge};


static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {

    let env = std::env::current_dir().unwrap();
    let current_dir = env.to_str().unwrap();

    let directory_state = use_context_provider(|| DirectoryContext { current_directory: Signal::new(current_dir.to_string()) });
    use_context_provider(|| DirectoryHistory { directory_history: Signal::new(vec![current_dir.to_string()]) });
    use_context_provider(|| ModeContext { mode: Signal::new(true) });

    initialize_bash();
    initialize_dioxus_bridge();

    rsx! {
        document::Stylesheet { href: CSS }
        div {
            class: "app-container",
            script { src: asset!("/assets/resizeBottom.js") },
            script { src: asset!("/assets/resizeSide.js") },
            SideBar {  } 
            TopBar { }
            FileExplorer { dir_path: directory_state.current_directory.read(), level: 0 }
            BottomBar {  }
            TerminalComponent { }
        }
    }
}