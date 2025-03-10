mod components;
mod utils;

use dioxus::prelude::*;
use components::{TopBar, SideBar, FileExplorer, TerminalComponent};
use utils::{DirectoryContext, DirectoryHistory, ModeContext};


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

    rsx! {
        document::Stylesheet { href: CSS }
        div {
            class: "app-container",
            script { src: asset!("/assets/bundled.js") },
            div { class: "side-bar", SideBar {  } }
            div { class: "top-bar", TopBar { }}
            div { class: "file-grid", FileExplorer { dir_path: directory_state.current_directory.read(), level: 0 }, TerminalComponent {  } }
        }
        //TerminalComponent { }
    }
}
