use dioxus::prelude::*;

static X_TERM: Asset = asset!("/assets/xterm.css");

#[component]
pub fn TerminalComponent() -> Element {
    let terminal_container_id = "terminal-div";

    rsx! {
        div { class: "bottom-component",
            document::Stylesheet { href: X_TERM }
            script { src: asset!("/assets/bundled.js") }
            div { style: "width: 100%; height: 100%;", id: "{terminal_container_id}"}
            // The terminal gets automatically fitted to the size of this parent div, due to a fitAddon feature in javascript.
        }
    }
}