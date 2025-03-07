mod components;
mod utils;

use dioxus::prelude::*;
use components::{TopBar, SideBar, FileExplorer};
use utils::{DirectoryContext, DirectoryHistory, ModeContext};

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {

    let env = std::env::current_dir().unwrap();
    let current_dir = env.to_str().unwrap();

    let directory_state = use_context_provider(|| DirectoryContext {
        current_directory: Signal::new(current_dir.to_string()),
    });

    use_context_provider(|| DirectoryHistory { // 
        directory_history: Signal::new(vec![current_dir.to_string()]),
    });

    use_context_provider(|| ModeContext {
        mode: Signal::new(true),
    });

    /*
        TODO list for app

        1. update the css and html structure so that it is actually making sense
        2. clean up syntax, make components clear and minimal

        Objectives:
        
        1. "Favourites" - be able to dynamically allocate favourite paths. Certain paths should have default icons (e.g. Downloads)
        2. "Icon Modes" - square view (current) // list view (was implemented but badly) - have these swappable via icons
        3. "Edit path" - have the path that is at the top be dynamically edittable, with error handling
        4. "Search" - a complicated one. For now, have it only search inside the folder you are in, or maybe up to 5 hierarchies.
        5. "Tags" - associate a tag with folders to quickly access them on the side bar, which makes them appear in the main folder view.

        6. Clean up UI and make it look neat
    
     */


    rsx! {
        document::Stylesheet { href: CSS }
        div {
            class: "app-container",
            div { class: "side-bar", SideBar {  } }
            div { class: "top-bar", TopBar { }}
            div { class: "file-grid", FileExplorer { dir_path: directory_state.current_directory.read(), level: 0 } }
        }
    }
}
