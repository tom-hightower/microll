use eframe::egui;

use crate::html;
use crate::navigation;
use crate::structs::{FileMenuState, State, WebpageType};

pub fn show_app_main_menu_bar(ui: &mut egui::Ui, state: &mut State) {
    egui::MenuBar::new().ui(ui, |ui| {
        ui.menu_button("File", |ui| show_main_menu_file(ui, state));
        ui.menu_button("View", |ui| show_main_menu_view(ui, &mut state.file_menu));
        ui.menu_button("Link", show_main_menu_link);
        ui.menu_button("Help", show_main_menu_help);
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
    let _ = ui.add(egui::Button::new("Go Back").shortcut_text("Alt+Left"));
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
                        state.main_body_array =
                            html::parse_html(&state.preloaded_pages[&finder.location]).0;
                    }
                }
            }
        }
    });
    let _ = ui.add(egui::Button::new("New Window").shortcut_text("Ctrl+N"));
    ui.separator();
    if ui
        .add(egui::Button::new("Open file").shortcut_text("Ctrl+O"))
        .clicked()
        && navigation::file_picker(state)
    {
        navigation::go_to_file(state);
    }
    let _ = ui.button("Save As");
    ui.separator();
    ui.checkbox(&mut state.file_menu.test_enabled, "Checked Test");
    if ui
        .add(egui::Button::new("Quit").shortcut_text("Alt+F4"))
        .clicked()
    {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    }
}

fn show_main_menu_view(ui: &mut egui::Ui, file_menu_state: &mut FileMenuState) {
    let _ = ui.add_enabled(
        file_menu_state.can_search,
        egui::Button::new("Search").shortcut_text("/"),
    );
    let _ = ui.add_enabled(
        file_menu_state.can_search,
        egui::Button::new("Search backward").shortcut_text("?"),
    );
    ui.separator();
    let _ = ui.add(egui::Button::new("Toggle HTML").shortcut_text("\\"));
    let _ = ui.add(egui::Button::new("Document Info").shortcut_text("="));
    ui.separator();
    let _ = ui.button("HTML Options");
}

fn show_main_menu_link(ui: &mut egui::Ui) {
    let _ = ui.add(egui::Button::new("Follow Link").shortcut_text("Right"));
    ui.separator();
    let _ = ui.add(egui::Button::new("Open in New Tab").shortcut_text("Ctrl+T"));
    let _ = ui.button("Open in New Background Tab");
    ui.separator();
    let _ = ui.button("Download Link");
}

fn show_main_menu_help(ui: &mut egui::Ui) {
    let _ = ui.add(egui::Button::new("Help").shortcut_text("F1"));
}
