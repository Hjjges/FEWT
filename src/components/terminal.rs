use dioxus::prelude::*;
use crate::utils::{TerminalState, initialize_dioxus_bridge, initialize_bash, send_command};

static X_TERM: Asset = asset!("/assets/xterm.css");

#[component]
pub fn TerminalComponent() -> Element {
    let terminal_container_id = "terminal-div";

    initialize_bash();
    initialize_dioxus_bridge();

    rsx! {
        document::Stylesheet { href: X_TERM }
        script { src: asset!("/assets/bundled.js") }
        div { style: "overflow-y: auto;", id: "{terminal_container_id}"}
    }
}