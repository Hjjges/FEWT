mod components;
mod utils;
mod helpers;

use dioxus::prelude::*;
use dioxus_desktop::use_muda_event_handler;
use components::{TopBar, SideBar, FileExplorer, TerminalComponent, BottomBar};
use utils::{initialize_bash, initialize_dioxus_bridge, AppConfig, Mode, DioxusContextMenu};
use directories::UserDirs;

static CSS: Asset = asset!("/assets/main.css");

// Globals
static DIR_HISTORY: GlobalSignal<Vec<String>> = Signal::global(|| vec![UserDirs::new().expect("Failed to get UserDirs").home_dir().to_string_lossy().into_owned()]);
static CURRENT_DIR: GlobalSignal<String> = Signal::global(|| UserDirs::new().expect("UserDirs failed").home_dir().to_string_lossy().into_owned());
static COPIED_PATH: GlobalSignal<String> = Signal::global(|| String::from(""));
static SORT: GlobalSignal<String> = Signal::global(|| String::from("name"));

//static mut INITIALIZED: bool = false;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let app_config = use_context_provider(|| Signal::new(AppConfig::load().expect("Failed to load config")));
    use_context_provider(|| Signal::new(Mode::Icon));

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

        // div { class: "file-explorer",
        // div { style: "display: grid; grid-auto-flow: column; grid-template-columns: repeat(auto-fit, 300px); height: 100vh; overflow-x: scroll;",
        //     div { style: "width: 300px; box-sizing: border-box; border-right: flex-shrink: 0; 1px solid #333; overflow: auto;", 
        //         div { style: "display: flex; flex-direction: column;",
        //             div { "hello" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }
        //         }
        //     }
        //     div { style: "width: 300px; box-sizing: border-box; border-right: 1px solid #333; overflow: auto;", 
        //         div { style: "display: flex; flex-direction: column;",
        //             div { "hello" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }
        //         }
        //     }
        //     div { style: "width: 300px; box-sizing: border-box; border-right: 1px solid #333; overflow: auto;", 
        //         div { style: "display: flex; flex-direction: column;",
        //             div { "hello" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }
        //         }
        //     }
        //     div { style: "width: 300px; box-sizing: border-box; border-right: 1px solid #333; overflow: auto;", 
        //     div { style: "display: flex; flex-direction: column;",
        //         div { "hello" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" } div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }div { "hello2" }
        //     }
    }
}
