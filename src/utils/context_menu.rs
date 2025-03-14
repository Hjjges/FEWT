use std::path::PathBuf;
use dioxus::prelude::*;

use dioxus_desktop::tao::platform::macos::WindowExtMacOS;

#[cfg(target_os = "macos")]
use cocoa::base::id;

use dioxus_desktop::muda::*;

pub struct DioxusContextMenu {
    menu: Menu,
}

#[derive(Clone)]
pub struct CurrentCopiedPath {
    pub path: Signal<String>,
}
  

impl DioxusContextMenu {
    // you can pass variables, such as the path of the file, which can be used to associate with the menu item.
    pub fn default() -> Self {
        let menu = Menu::new();
        let option_copy = MenuItem::with_id("anton-copies", "Copy", true, None);
        let option_paste = MenuItem::with_id("anton-pastes", "Paste", true, None);
        let option_favourites = MenuItem::with_id("add-favourite", "Add to Favourites", true, None);
        let option_quick_access = MenuItem::with_id("add-quick", "Add to Quick Access", true, None);


        menu.append_items(&[
            &option_copy,
            &option_paste,
            &option_favourites,
            &option_quick_access,
        ]).unwrap();
        let ns_view = dioxus_desktop::use_window().ns_view();
        let ns_view_id: id = unsafe { std::mem::transmute(ns_view) };
        menu.show_context_menu_for_nsview(ns_view_id, None);
        Self { menu }
    }
}