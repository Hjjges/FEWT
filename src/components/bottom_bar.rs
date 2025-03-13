use dioxus::prelude::*;

use crate::components::ResizerBar;

#[component]
pub fn BottomBar() -> Element {
    rsx! {
        div { class: "bottom-bar", 
            ResizerBar { class: "resizer-bottom", custom_style: "height: 10px;"}
            div { class: "bottom-bar-inner",
                BottomBarButton { header: "TERMINAL" }
                BottomBarButton { header: "OUTPUT" }
                BottomBarButton { header: "PROBLEMS" }
            }
        }
    }
}

#[component]
fn BottomBarButton(header: String) -> Element {

    let mut selected = use_signal(|| false);
    let mut hover = use_signal(|| false);
    let mut terminal_only = use_signal(|| false);
    let border = use_memo(move || if selected() { "1px solid turquoise" } else { "0" });
    let color = use_memo(move || if selected() || hover() { "white" } else { "gray" });

    rsx! {
        div {
            class: "bottom-bar-button",
            style: "color: {color}; border-bottom: {border};", 
            onclick: move |_| {
                selected.set(!selected());
                if header.starts_with("TERMINAL") && !terminal_only() { 
                    println!("toggling"); 
                    document::eval(r#"window.initTerminal('terminal-div')"#);
                    terminal_only.set(true); 
                };
                if selected() { document::eval(r#"window.showTerminal('terminal-div')"#); } else { document::eval(r#"window.hideTerminal('terminal-div')"#);}
            },
            onmouseenter: move |_| hover.set(true),
            onmouseleave: move |_| hover.set(false),
            "{header}"
        }
    }
}