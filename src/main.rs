mod components;
mod utils;
mod helpers;

use dioxus::{logger::tracing, prelude::*};
use arboard::Clipboard;
use arboard::ImageData;
use std::borrow::Cow;
use image::GenericImageView;
use std::path::*;
use components::{TopBar, SideBar, FileExplorer, TerminalComponent, BottomBar};
use utils::{initialize_bash, initialize_dioxus_bridge, AppConfig, DirectoryContext, DirectoryHistory, ModeContext, CurrentCopiedPath, DioxusContextMenu};
use dioxus_desktop::use_muda_event_handler;

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
        Err(e) => { tracing::error!("Error loading config: {}", e); AppConfig::default() }
    };

    // Testing to see if app config global provider works
    use_context_provider(|| app_config);
    let test = use_context::<AppConfig>().quick_access;
    tracing::info!("{:#?}", test);
    // End testing


    // These need to get evaluated because they are re-running on each app render, which is obviously not what
    initialize_bash();
    initialize_dioxus_bridge();


    let directory_state = use_context_provider(|| DirectoryContext { current_directory: Signal::new(current_dir.to_string()) });
    use_context_provider(|| DirectoryHistory { directory_history: Signal::new(vec![current_dir.to_string()]) });
    use_context_provider(|| ModeContext { mode: Signal::new(true) });
    use_context_provider(|| CurrentCopiedPath { path: Signal::new(String::from("hello")) });

    use_muda_event_handler(move |muda_event| {
        let path = use_context::<CurrentCopiedPath>().path.read().as_str().to_string();
        tracing::info!(path);
        if muda_event.id() == "anton-copies" {
            // Testing clipboard
            let test = PathBuf::from(&path);
            let mut clipboard = Clipboard::new().unwrap();


            let img_file = image::open(&test).expect("Failed to open image");

            let dimensions = img_file.dimensions();
            let rgba_image = img_file.to_rgba8();
            let raw_bytes = rgba_image.into_raw();
            let img = ImageData {
                width: dimensions.0 as usize,
                height: dimensions.1 as usize,
                bytes: Cow::from(raw_bytes)
            };

            clipboard.set_image(img).expect("Failed to copy image");
        } else if muda_event.id() == "anton-pastes" {
        } else if muda_event.id() == "add-favourite" {
            use_context::<AppConfig>().add_favorite(PathBuf::from(&path));
            use_context::<AppConfig>().save().expect("failed to save");
        } else if muda_event.id() == "add-quick" {
            // use_context::<AppConfig>().add_(PathBuf::from(&path));
            // use_context::<AppConfig>().save();
        }
    });

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