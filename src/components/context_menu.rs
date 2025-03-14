use dioxus::prelude::*;
#[component]
pub fn ContextMenu(menu_visible: Signal<bool>, menu_position: Signal<(f64, f64)>) -> Element {
    rsx! {
        div { class: "context-menu",
                style: format!(
                    "left: {}px; top: {}px;",
                    menu_position().0, menu_position().1
                ),
                onclick: move |_| menu_visible.set(false), // Hide menu on click
                span { style: "padding: 3px", "Copy" }
                span { style: "padding: 3px", "Set to terminal entry" } 
            }
    }
}