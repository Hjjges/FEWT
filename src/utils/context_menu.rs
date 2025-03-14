use dioxus_desktop::tao::platform::macos::WindowExtMacOS;
use dioxus_desktop::muda::{self as muda, ContextMenu};
use muda::{Menu, MenuItem, MenuEvent, PredefinedMenuItem};

#[cfg(target_os = "macos")]
use cocoa::base::id;

pub struct DioxusContextMenu {

}

impl Default for DioxusContextMenu {
    fn default() -> Self {
        let menu = Menu::new();
        let copy_item = MenuItem::new("Copy", true, None);
        let paste_item = MenuItem::new("Paste", true, None);
        let separator = PredefinedMenuItem::separator();
        menu.append(&copy_item).unwrap();
        menu.append(&paste_item).unwrap();
        menu.append(&separator).unwrap();
        let ns_view = dioxus_desktop::use_window().ns_view();
        let ns_view_id: id = unsafe { std::mem::transmute(ns_view) };
        menu.show_context_menu_for_nsview(ns_view_id, None);
        Self { }
    }
}