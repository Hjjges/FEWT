use dioxus_desktop::tao::platform::macos::WindowExtMacOS;

#[cfg(target_os = "macos")]
use cocoa::base::id;

use dioxus_desktop::muda::*;

pub struct DioxusContextMenu {
}
  

impl Default for DioxusContextMenu {
    fn default() -> Self {
        let menu = Menu::new();
        let option_copy = MenuItem::with_id("anton-copies", "Copy", true, None);
        let option_paste = MenuItem::with_id("anton-pastes", "Paste", true, None);
        menu.append_items(&[
            &option_copy,
            &option_paste,
        ]).unwrap();
        let ns_view = dioxus_desktop::use_window().ns_view();
        let ns_view_id: id = unsafe { std::mem::transmute(ns_view) };
        menu.show_context_menu_for_nsview(ns_view_id, None);
        Self { }
    }
}