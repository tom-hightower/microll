use imgui::*;
use std::process;

use crate::html;
use crate::navigation;
use crate::structs::{FileMenuState, State, WebpageType};

pub fn show_app_main_menu_bar<'a>(ui: &Ui<'a>, state: &mut State, dimensions: (u32, u32)) {
    if let Some(menu_bar) = ui.begin_main_menu_bar() {
        if let Some(menu) = ui.begin_menu(im_str!("File"), true) {
            show_main_menu_file(ui, state);
            menu.end(ui);
        }
        if let Some(menu) = ui.begin_menu(im_str!("View"), true) {
            show_main_menu_view(ui, &mut state.file_menu);
            menu.end(ui);
        }
        if let Some(menu) = ui.begin_menu(im_str!("Link"), true) {
            show_main_menu_link(ui);
            menu.end(ui);
        }
        if let Some(menu) = ui.begin_menu(im_str!("Help"), true) {
            show_main_menu_help(ui);
            menu.end(ui);
        }
        ui.same_line(
            (dimensions.0 as f32)
                - (ui.calc_text_size(&im_str!("{:}", state.window_title), false, 0.)[0] + 20.),
        );
        ui.text(im_str!("{:}", state.window_title));
        menu_bar.end(ui);
    }
}

fn show_main_menu_file<'a>(ui: &Ui<'a>, state: &mut State) {
    if MenuItem::new(im_str!("Go to URL"))
        .shortcut(im_str!("Ctrl+G"))
        .build(ui)
    {
        state.sub_windows.go_to_link = !state.sub_windows.go_to_link;
    }
    MenuItem::new(im_str!("Go Back"))
        .shortcut(im_str!("Alt+LEFT"))
        .build(ui);
    if let Some(menu) = ui.begin_menu(im_str!("Recent"), true) {
        for (name, finder) in &state.history.clone() {
            if MenuItem::new(&ImString::new(name)).build(ui) {
                match finder.web_type {
                    WebpageType::File => {
                        state.file_menu.file_to_get = finder.location.clone();
                        navigation::go_to_file(state);
                    }
                    WebpageType::Link => {
                        state.url_to_get = ImString::from(finder.location.clone());
                        navigation::go_to_page(state);
                    }
                    WebpageType::Preload => {
                        state.main_body_array =
                            html::parse_html(&state.preloaded_pages[&finder.location]).0;
                    }
                }
            }
        }
        menu.end(ui);
    }
    MenuItem::new(im_str!("New Window"))
        .shortcut(im_str!("Ctrl+N"))
        .build(ui);
    ui.separator();
    if MenuItem::new(im_str!("Open file"))
        .shortcut(im_str!("Ctrl+O"))
        .build(ui)
    {
        if navigation::file_picker(state) {
            navigation::go_to_file(state);
        }
    }
    MenuItem::new(im_str!("Save As")).build(ui);
    ui.separator();
    if let Some(menu) = ui.begin_menu(im_str!("Colors"), true) {
        for &col in StyleColor::VARIANTS.iter() {
            MenuItem::new(&im_str!("{:?}", col)).build(ui);
        }
        menu.end(ui);
    }
    if MenuItem::new(im_str!("Checked Test"))
        .selected(state.file_menu.test_enabled)
        .build(ui)
    {
        state.file_menu.test_enabled = !state.file_menu.test_enabled;
    }
    if MenuItem::new(im_str!("Quit"))
        .shortcut(im_str!("Alt+F4"))
        .build(ui)
    {
        process::exit(0x0000);
    }
}

fn show_main_menu_view<'a>(ui: &Ui<'a>, file_menu_state: &mut FileMenuState) {
    MenuItem::new(im_str!("Search"))
        .enabled(file_menu_state.can_search)
        .shortcut(im_str!("/"))
        .build(ui);
    MenuItem::new(im_str!("Search backward"))
        .enabled(file_menu_state.can_search)
        .shortcut(im_str!("?"))
        .build(ui);
    ui.separator();
    MenuItem::new(im_str!("Toggle HTML"))
        .shortcut(im_str!("\\"))
        .build(ui);
    MenuItem::new(im_str!("Document Info"))
        .shortcut(im_str!("="))
        .build(ui);
    ui.separator();
    MenuItem::new(im_str!("HTML Options")).build(ui);
}

fn show_main_menu_link<'a>(ui: &Ui<'a>) {
    MenuItem::new(im_str!("Follow Link"))
        .shortcut(im_str!("Right"))
        .build(ui);
    ui.separator();
    MenuItem::new(im_str!("Open in New Tab"))
        .shortcut(im_str!("Ctrl+T"))
        .build(ui);
    MenuItem::new(im_str!("Open in New Background Tab")).build(ui);
    ui.separator();
    MenuItem::new(im_str!("Download Link")).build(ui);
}

fn show_main_menu_help<'a>(ui: &Ui<'a>) {
    MenuItem::new(im_str!("Help"))
        .shortcut(im_str!("F1"))
        .build(ui);
}
