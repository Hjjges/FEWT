mod components;
mod utils;
mod helpers;

use dioxus::prelude::*;
use components::{TopBar, SideBar, FileExplorer, TerminalComponent, BottomBar};
use utils::{initialize_bash, initialize_dioxus_bridge, AppConfig, DirectoryContext, DirectoryHistory, ModeContext};

static CSS: Asset = asset!("/assets/main.css");

fn main() {

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut initialized = use_signal(|| false);

    let env = std::env::current_dir().unwrap();
    let current_dir = env.to_str().unwrap();

    // Testing loading app config
    let app_config = match AppConfig::load() {
        Ok(config) => config,
        Err(e) => { eprintln!("Error loading config: {}", e); AppConfig::default() }
    };

    // Testing to see if app config global provider works
    use_context_provider(|| app_config);
    let test = use_context::<AppConfig>().quick_access;
    println!("{:#?}", test);
    // End testing

    initialize_bash();
    initialize_dioxus_bridge();


    let directory_state = use_context_provider(|| DirectoryContext { current_directory: Signal::new(current_dir.to_string()) });
    use_context_provider(|| DirectoryHistory { directory_history: Signal::new(vec![current_dir.to_string()]) });
    use_context_provider(|| ModeContext { mode: Signal::new(true) });

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