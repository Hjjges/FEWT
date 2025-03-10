use anyhow::Result;
use dioxus::{document::Eval, html::g::dangerous_inner_html, prelude::*};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use wasmer::{Store, Module, Instance, Value, imports};
use wasmer_wasi::{WasiState, WasiEnv, WasiFunctionEnv};

static X_TERM: Asset = asset!("/assets/xterm.css");

#[component]
pub fn TerminalComponent() -> Element {
    println!("Built");

    let mut terminal_state = use_signal(|| {
        TerminalState::new().unwrap_or_else(|e| {
            eprintln!("Failed to start terminal: {}", e);
            std::process::exit(1);
        })
    });
    let terminal_container_id = "terminal-div";

    // let js_comms = use_resource(move || async move {
    //     let mut eval = document::eval(
    //         r#"
    //         // You can send messages from JavaScript to Rust with the dioxus.send function
    //         dioxus.send("Hi from JS! - Spawning terminal");
    //         // You can receive messages from Rust to JavaScript with the dioxus.recv function
    //         let msg = await dioxus.recv();
    //         console.log(msg);
    //         "#,
    //     );

    //     eval.send("Hello from rust babe").unwrap();

    //     println!("{}", eval.recv::<String>().await.unwrap());
    // });

    // match js_comms.read_unchecked().as_ref() {
    //     Some(v) => rsx! {
    //         document::Stylesheet { href: X_TERM }
    //         script { src: asset!("/assets/bundled.js") }
    //         div { class: "", style: "overflow-y: auto;", id: "{terminal_container_id}"}
    //     },
    //     _ => rsx! {
    //         document::Stylesheet { href: X_TERM }
    //         script { src: asset!("/assets/bundled.js") }
    //         div { class: "", style: "overflow-y: auto;", id: "{terminal_container_id}"}
    //     },
    // }


    // let future = use_resource(move || {
    //     async move { 
    //         let mut eval = document::eval(
    //             r#"console.log(document.getElementById('terminal-div')); console.log('reached'); window.initTerminal('terminal-div');"#,
    //         );

    //         while let Ok(command) = eval.recv::<String>().await {
    //             // Execute the command
    //             let result = {
    //                 let mut fs = terminal_state.write();
    //                 fs.execute_command(&command)
    //             };
                
    //             // Send the result back to the terminal
    //             let _ = eval.send(result);
    //         }
    //     }
    // });

    
    rsx! {
        document::Stylesheet { href: X_TERM }
        script { src: asset!("/assets/bundled.js") }
        div { class: "", style: "overflow-y: auto;", id: "{terminal_container_id}"}
    }
}

// maybe update this code later to use signals isntead of terminal state, unless we combine the structs with the signals.
struct TerminalState {
    current_dir: std::path::PathBuf,
    wasmer_store: Store,
    wasi_env: WasiFunctionEnv,
}

impl TerminalState {
    fn new() -> Result<Self> {
        // Initilaize the current directory
        let current_dir = std::env::current_dir()?;

        // Initialise Wasmer
        let mut store = Store::default();

        let wasi_env = WasiState::new("terminal-user")
            .preopen_dir(current_dir.clone())?
            .finalize(&mut store)?;


        Ok(Self {
            current_dir,
            wasmer_store: store,
            wasi_env,
        })
    }
}

//     fn execute_command(&mut self, command: &str) -> String {
//         let parts: Vec<&str> = command.trim().split_whitespace().collect();

//         if parts.is_empty() {
//             return String::new();
//         }

//         match parts[0] {
//             "cd" => {
//                 let new_dir = parts.get(1).unwrap_or(&".");
//                 match self.change_directory(new_dir) {
//                     Ok(_) => format!("Changed directory to {}", self.current_dir.display()),
//                     Err(e) => format!("Error: {}", e)
//                 }
//             },
//             "pwd" => {
//                 self.current_dir.display().to_string()
//             },
//             _ => format!("Command not found {}", parts[0])
//         }
//     }

//     fn change_directory(&mut self, path: &str) -> Result<(), std::io::Error> {
//         let new_path = if path.starts_with('/') {
//             std::path::PathBuf::from(path)
//         } else {
//             self.current_dir.join(path)
//         };

//         if new_path.exists() && new_path.is_dir() {
//             self.current_dir = new_path.canonicalize()?;
//             Ok(())
//         } else {
//             Err(std::io::Error::new(
//                 std::io::ErrorKind::NotFound,
//                 "Directory does not exist"
//             ))
//         }
//     }
// }
