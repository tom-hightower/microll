use imgui::*;

use crate::structs::State;
use crate::structs::FileMenuState;

pub fn show_app_main_menu_bar<'a>(ui: &Ui<'a>, state: &mut State) {
    ui.main_menu_bar(|| {
        ui.menu(im_str!("File")).build(|| {
            show_main_menu_file(ui, &mut state.file_menu);
        });
        ui.menu(im_str!("View")).build(|| {
            show_main_menu_view(ui, &mut state.file_menu);
        });
        ui.menu(im_str!("Link")).build(|| {
            show_main_menu_link(ui);
        });
        ui.menu(im_str!("Help")).build(|| {
            show_main_menu_help(ui);
        });
    });
}

fn show_main_menu_file<'a>(ui: &Ui<'a>, state: &mut FileMenuState) {
    ui.menu_item(im_str!("Go to URL"))
        .shortcut(im_str!("Ctrl+G"))
        .build();
    ui.menu_item(im_str!("Go Back"))
        .shortcut(im_str!("Alt+LEFT"))
        .build();
    ui.menu(im_str!("Recent")).build(|| {
        ui.menu_item(im_str!("Site 1")).build();
        ui.menu_item(im_str!("Site 2")).build();
        ui.menu_item(im_str!("Site 3")).build();
    });
    ui.menu_item(im_str!("New Window"))
        .shortcut(im_str!("Ctrl+N"))
        .build();
    ui.separator();
    ui.menu_item(im_str!("Save As")).build();
    ui.separator();
    ui.menu(im_str!("Colors")).build(|| {
        for &col in StyleColor::VARIANTS.iter() {
            ui.menu_item(&im_str!("{:?}", col)).build();
        }
    });
    ui.menu_item(im_str!("Checked Test"))
        .selected(&mut state.test_enabled)
        .build();
    ui.menu_item(im_str!("Quit"))
        .shortcut(im_str!("Alt+F4"))
        .build();
}

fn show_main_menu_view<'a>(ui: &Ui<'a>, state: &mut FileMenuState) {
    ui.menu_item(im_str!("Search"))
        .enabled(state.can_search)
        .shortcut(im_str!("/"))
        .build();
    ui.menu_item(im_str!("Search backward"))
        .enabled(state.can_search)
        .shortcut(im_str!("?"))
        .build();
    ui.separator();
    ui.menu_item(im_str!("Toggle HTML"))
        .shortcut(im_str!("\\"))
        .build();
    ui.menu_item(im_str!("Document Info"))
        .shortcut(im_str!("="))
        .build();
    ui.separator();
    ui.menu_item(im_str!("HTML Options")).build();
}

fn show_main_menu_link<'a>(ui: &Ui<'a>) {
    ui.menu_item(im_str!("Follow Link"))
        .shortcut(im_str!("Right"))
        .build();
    ui.separator();
    ui.menu_item(im_str!("Open in New Tab"))
        .shortcut(im_str!("Ctrl+T"))
        .build();
    ui.menu_item(im_str!("Open in New Background Tab")).build();
    ui.separator();
    ui.menu_item(im_str!("Download Link")).build();
}

fn show_main_menu_help<'a>(ui: &Ui<'a>) {
    ui.menu_item(im_str!("Help"))
        .shortcut(im_str!("F1"))
        .build();
}