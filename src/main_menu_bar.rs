use conrod::*;
use std::process;

use crate::conrod_ids::Ids;
use crate::html;
use crate::navigation;
use crate::structs::{FileMenuState, State, WebpageType};

pub fn show_app_main_menu_bar<'a>(ui: &mut UiCell, state: &mut State, ids: &mut Ids) {
    widget::Tabs::new(&[
        (ids.menu_bar.file_menu.button, "File"),
        (ids.menu_bar.view_menu.button, "View"),
        (ids.menu_bar.link_menu.button, "Link"),
        (ids.menu_bar.help_menu.button, "Help"),
    ])
    .parent(ids.menu_bar.canvas)
    .label_color(color::WHITE)
    .label_font_size(10)
    .color(color::DARK_CHARCOAL)
    .border(0.)
    .w_h(300., ui.wh_of(ids.menu_bar.canvas).unwrap()[1])
    .mid_left_of(ids.menu_bar.canvas)
    .set(ids.menu_bar.tabs, ui);

    if is_tab_clicked(ui, ids.menu_bar.file_menu.button) {
        show_main_menu_file(ui, state, ids);
    }

    if is_tab_clicked(ui, ids.menu_bar.view_menu.button) {
        show_main_menu_view(ui, state, ids);
    }

    if is_tab_clicked(ui, ids.menu_bar.link_menu.button) {
        show_main_menu_link(ui, state, ids);
    }

    if is_tab_clicked(ui, ids.menu_bar.help_menu.button) {
        show_main_menu_help(ui, state, ids);
    }

    widget::Text::new(&state.window_title)
        .parent(ids.url_bar.canvas)
        .mid_right_of(ids.menu_bar.canvas)
        .set(ids.menu_bar.cur_page, ui);
}

fn show_main_menu_file<'a>(ui: &mut UiCell, state: &mut State, ids: &mut Ids) {
    /*
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
    */
}

fn show_main_menu_view<'a>(ui: &mut UiCell, state: &mut State, ids: &mut Ids) {
    /*
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
    */
}

fn show_main_menu_link<'a>(ui: &mut UiCell, state: &mut State, ids: &mut Ids) {
    /*
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
    */
}

fn show_main_menu_help<'a>(ui: &mut UiCell, state: &mut State, ids: &mut Ids) {
    /*
        MenuItem::new(im_str!("Help"))
            .shortcut(im_str!("F1"))
            .build(ui);
    */
}

fn is_tab_clicked(ui: &mut UiCell, id: widget::Id) -> bool {
    ui.widget_input(id).clicks().right().count() != 0
}
