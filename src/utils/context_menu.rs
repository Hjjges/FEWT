use dioxus::prelude::*;
use dioxus_desktop::{tao::platform::macos::WindowExtMacOS, muda::*};
use crate::utils::{AppConfig, Genre};
use crate::COPIED_PATH;
use arboard::Clipboard;
use arboard::ImageData;
use std::borrow::Cow;
use std::path::PathBuf;
use image::GenericImageView;

#[cfg(target_os = "macos")]
use cocoa::base::id;

pub struct DioxusContextMenu {
    menu: Menu,
}

impl DioxusContextMenu {
    pub fn default() -> Self {
        let menu = Menu::new();
        let option_copy = MenuItem::with_id("copy", "Copy", true, None);
        let option_paste = MenuItem::with_id("paste", "Paste", true, None);
        let option_favourites = MenuItem::with_id("add-favourite", "Add to Favourites", true, None);
        let option_quick_access = MenuItem::with_id("add-quick-access", "Add to Quick Access", true, None);

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

    pub fn side_bar(category: Genre) -> Self {
        let menu = Menu::new();
        let id = match category { Genre::Favourites => "remove-favourite", Genre::QuickAccess => "remove-quick-access"};
        let text = match category { Genre::Favourites => "Remove from Favourites", Genre::QuickAccess => "Remove from Quick Access"};
        let option = MenuItem::with_id(id, text, true, None);
        menu.append_items(&[
            &option,
        ]).unwrap();
        let ns_view = dioxus_desktop::use_window().ns_view();
        let ns_view_id: id = unsafe { std::mem::transmute(ns_view) };
        menu.show_context_menu_for_nsview(ns_view_id, None);
        Self { menu }
    }

    pub fn muda_handler(app_config: Signal<AppConfig>, muda_event: &MenuEvent) {
        let copied_path = PathBuf::from(COPIED_PATH()); 
        match muda_event.id().0.as_str() {
            "copy" => {
                let mut clipboard = Clipboard::new().unwrap();
                let img_file = image::open(&copied_path).expect("Failed to open image");
    
                let dimensions = img_file.dimensions();
                let rgba_image = img_file.to_rgba8();
                let raw_bytes = rgba_image.into_raw();
                let img = ImageData {
                    width: dimensions.0 as usize,
                    height: dimensions.1 as usize,
                    bytes: Cow::from(raw_bytes),
                };
    
                clipboard.set_image(img).expect("Failed to copy image");
            }
            "add-favourite" => app_config().add_entry(copied_path, Genre::Favourites),
            "add-quick-access" => app_config().add_entry(copied_path, Genre::QuickAccess),
            "remove-favourite" => app_config().remove_entry(copied_path, Genre::Favourites),
            "remove-quick-access" => app_config().remove_entry(copied_path, Genre::QuickAccess),
            _ => {} // Handle unknown events gracefully
        }
    }
}