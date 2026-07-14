use eframe::egui;

use crate::navigation;
use crate::structs::{State, WebpageType};

pub fn show_app_main_menu_bar(ui: &mut egui::Ui, state: &mut State) {
    egui::MenuBar::new().ui(ui, |ui| {
        ui.menu_button("File", |ui| show_main_menu_file(ui, state));
        ui.menu_button("View", |ui| show_main_menu_view(ui, state));
        ui.menu_button("Help", |ui| show_main_menu_help(ui, state));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(&state.window_title);
        });
    });
}

fn show_main_menu_file(ui: &mut egui::Ui, state: &mut State) {
    if ui
        .add(egui::Button::new("Go to URL").shortcut_text("Ctrl+G"))
        .clicked()
    {
        state.sub_windows.go_to_link = !state.sub_windows.go_to_link;
    }
    if ui
        .add_enabled(
            !state.back_stack.is_empty(),
            egui::Button::new("Go Back").shortcut_text("Alt+Left"),
        )
        .clicked()
    {
        navigation::go_back(state);
    }
    ui.menu_button("Recent", |ui| {
        for (name, finder) in &state.history.clone() {
            if ui.button(name).clicked() {
                match finder.web_type {
                    WebpageType::File => {
                        state.file_menu.file_to_get = finder.location.clone();
                        navigation::go_to_file(state);
                    }
                    WebpageType::Link => {
                        state.url_to_get = finder.location.clone();
                        navigation::go_to_page(state);
                    }
                    WebpageType::Preload => {
                        navigation::go_to_preload(state, &finder.location);
                    }
                }
            }
        }
    });
    ui.add_enabled(
        false,
        egui::Button::new("New Window").shortcut_text("Ctrl+N"),
    )
    .on_disabled_hover_text("Not yet implemented");
    ui.separator();
    if ui
        .add(egui::Button::new("Open file").shortcut_text("Ctrl+O"))
        .clicked()
        && navigation::file_picker(state)
    {
        navigation::go_to_file(state);
    }
    if ui.button("Save As").clicked() {
        navigation::save_as(state);
    }
    ui.separator();
    if ui
        .add(egui::Button::new("Quit").shortcut_text("Alt+F4"))
        .clicked()
    {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    }
}

fn show_main_menu_view(ui: &mut egui::Ui, state: &mut State) {
    if ui
        .add_enabled(
            state.file_menu.can_search,
            egui::Button::new("Search").shortcut_text("/"),
        )
        .clicked()
    {
        state.sub_windows.search = true;
    }
    if ui
        .add_enabled(
            state.file_menu.can_search,
            egui::Button::new("Search backward").shortcut_text("?"),
        )
        .clicked()
    {
        state.sub_windows.search = true;
        navigation::step_search(state, false);
    }
    ui.separator();
    if ui
        .add(egui::Button::new("Toggle HTML").shortcut_text("\\"))
        .clicked()
    {
        state.sub_windows.show_raw_html = !state.sub_windows.show_raw_html;
    }
    if ui
        .add(egui::Button::new("Document Info").shortcut_text("="))
        .clicked()
    {
        state.sub_windows.document_info = true;
    }
}

fn show_main_menu_help(ui: &mut egui::Ui, state: &mut State) {
    if ui
        .add(egui::Button::new("Help").shortcut_text("F1"))
        .clicked()
    {
        state.sub_windows.help = true;
    }
}
