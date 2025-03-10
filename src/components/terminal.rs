use anyhow::Result;
use dioxus::{document::Eval, html::g::dangerous_inner_html, prelude::*};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use wasmer::{Store, Module, Instance, Value, imports};
use wasmer_wasi::{WasiState, WasiEnv, WasiFunctionEnv};

static X_TERM: Asset = asset!("/assets/xterm.css");


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


#[component]
pub fn TerminalComponent() -> Element {
    println!("Built");

    let terminal_state = use_signal(|| {
        TerminalState::new().unwrap_or_else(|e| {
            eprintln!("Failed to start terminal: {}", e);
            std::process::exit(1);
        })
    });

    let terminal_container_id = "terminal-div";

    let mut eval = document::eval(
        r#"
        console.log('crazy');
        initTerminal('terminal-div');
        console.log('should have called terminal')
        "#,
    );

    rsx! {
        document::Stylesheet { href: X_TERM }
        script { src: asset!("/assets/bundled.js") }
        div { style: "width: 100%; height: 800px; background-color: black", id: "{terminal_container_id}" }
    }
}