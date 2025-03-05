use dioxus::prelude::*;
use std::path::PathBuf;

mod components;
mod utils;

use components::SideBar;
use components::FileExplorer;
use utils::DirectoryContext;
use utils::ModeContext;

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {

    let directory_state= use_context_provider(|| DirectoryContext {
        current_directory: Signal::new(PathBuf::from("/Users/hgregory/Documents")),
    });

    use_context_provider(|| ModeContext { // store in variable if mode state needs to be used in this component, but for now its not
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
        div { style: "display: flex;",
            SideBar {  }
            FileExplorer { 
                dir_path: directory_state.current_directory.read().clone().to_string_lossy(),
                level: 0,
            }
        }
    }
}
