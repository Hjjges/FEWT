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

    use_effect(move || {
        initialize_dioxus_bridge();
    });

    rsx! {
        document::Stylesheet { href: CSS }
        div {
            class: "app-container",
            script { src: asset!("/assets/bundled.js") },
            div { class: "side-bar", SideBar {  } }
            div { class: "top-bar", TopBar { } }
            div { class: "file-grid", FileExplorer { dir_path: directory_state.current_directory.read(), level: 0 } }
            div { class: "terminal-grid", TerminalComponent {  } }
        }
    }
}

fn initialize_dioxus_bridge() -> Task {
    let bridge_script = r#"
    window.dioxusBridge = {
        toRustQueue: [],
        fromRustQueue: [],
        waitCallbacks: [],

        sendToRust: function(message) {
            this.toRustQueue.push(message);
        },

        receiveFromRust: function() {
            return new Promise((resolve) => {
                if (this.fromRustQueue.length > 0) {
                    resolve(this.fromRustQueue.shift());
                } else {
                    this.waitCallbacks.push(resolve);
                }
            });
        }
    }

    console.log("Dioxus Bridge Initialized");
    "#;

    document::eval(bridge_script);

    spawn(async move {
        let mut eval = document::eval(r#"
        async function processToRustQueue() {
            while (true) {
                if (window.dioxusBridge && window.dioxusBridge.toRustQueue.length > 0) {
                    const message = window.dioxusBridge.toRustQueue.shift();
                    console.log("Sending to Rust:", message);
                    dioxus.send(message);
                }
                // Wait
                await new Promise(resolve => setTimeout(resolve, 100));
            }
        }

        processToRustQueue();

        "Bridge communication started"
        "#);

        while let Ok(message) = eval.recv::<String>().await {
            println!("Received from JS: {}", message);
            
            // // Process the message (replace with your actual command handling)
            // let response = format!("Processed: {}", message);
            
            // // Send the response back to JavaScript
            // let send_script = format!(
            //     r#"
            //     // Add the response to the fromRustQueue
            //     if (window.dioxusBridge) {{
            //         const response = {};
            //         console.log("Received from Rust:", response);
                    
            //         // If there are callbacks waiting, resolve the first one
            //         if (window.dioxusBridge.waitCallbacks.length > 0) {{
            //             const callback = window.dioxusBridge.waitCallbacks.shift();
            //             callback(response);
            //         }} else {{
            //             // Otherwise, add to the queue
            //             window.dioxusBridge.fromRustQueue.push(response);
            //         }}
            //     }}
            //     "#,
            //     serde_json::to_string(&response).unwrap()
            // );
            
            // document::eval(&send_script);
        }
    })
}