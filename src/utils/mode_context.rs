use dioxus::prelude::*;

 #[derive(Clone, Copy)]
pub struct ModeContext {
    pub mode: Signal<bool>,
}