mod components;
mod utils;
mod helpers;

use dioxus::prelude::*;
use dioxus_desktop::use_muda_event_handler;
use components::{TopBar, SideBar, FileExplorer, TerminalComponent, BottomBar};
use utils::{initialize_bash, initialize_dioxus_bridge, AppConfig, DioxusContextMenu};
use directories::UserDirs;

static CSS: Asset = asset!("/assets/main.css");

// Globals
static DIR_HISTORY: GlobalSignal<Vec<String>> = Signal::global(|| vec![UserDirs::new().expect("Failed to get UserDirs").home_dir().to_string_lossy().into_owned()]);
static CURRENT_DIR: GlobalSignal<String> = Signal::global(|| UserDirs::new().expect("UserDirs failed").home_dir().to_string_lossy().into_owned());
static CURRENT_MODE: GlobalSignal<bool> = Signal::global(|| false);
static COPIED_PATH: GlobalSignal<String> = Signal::global(|| String::from(""));
//static mut INITIALIZED: bool = false;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let app_config = use_context_provider(|| Signal::new(AppConfig::load().expect("Failed to load config")));

    initialize_bash();
    initialize_dioxus_bridge();

    use_muda_event_handler(move |muda_event| {
        DioxusContextMenu::muda_handler(app_config, muda_event);
    });

    rsx! {
        document::Stylesheet { href: CSS }
        div {
            class: "app-container",
            script { src: asset!("/assets/resizeBottom.js") },
            script { src: asset!("/assets/resizeSide.js") },
            SideBar {  } 
            TopBar { }
            FileExplorer { dir_path: CURRENT_DIR(), level: 0 }
            BottomBar {  }
            TerminalComponent { }
        }
    }
}
