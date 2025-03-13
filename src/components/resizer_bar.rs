use dioxus::prelude::*;

#[component]
pub fn ResizerBar(class: String, custom_style: String) -> Element {
    let mut hover = use_signal(|| false);
    let mut dragging = use_signal(|| false);
    let background_color = use_memo(move || if hover() || dragging() { "turquoise" } else { "#1f1f1f" });
    let cursor = use_memo(move || if hover() || dragging() { "ns-resize" } else { "default" });
    rsx! {
        div {
            class: "{class} resizer-default",
            style: "background-color: {background_color}; width: 100%; cursor: {cursor}; z-index: 3; {custom_style}",
            onmouseenter: move |_| hover.set(true),
            onmouseleave: move |_| hover.set(false),
            onmousedown: move |_| dragging.set(true),
            onmouseup: move |_| dragging.set(false)
        }
    }
}