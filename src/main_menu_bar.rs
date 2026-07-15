use eframe::egui;

use crate::menu_actions::{self, MenuAction};
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

fn action_button(ui: &mut egui::Ui, state: &mut State, action: MenuAction) {
    let mut button = egui::Button::new(action.label());
    if let Some(shortcut) = action.shortcut() {
        button = button.shortcut_text(menu_actions::format_shortcut(&shortcut));
    }
    if ui.add_enabled(action.enabled(state), button).clicked() {
        action.apply(state, ui.ctx());
    }
}

fn show_main_menu_file(ui: &mut egui::Ui, state: &mut State) {
    action_button(ui, state, MenuAction::GoToUrl);
    action_button(ui, state, MenuAction::GoBack);
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
    action_button(ui, state, MenuAction::OpenFile);
    if ui.button("Save As").clicked() {
        navigation::save_as(state);
    }
    ui.separator();
    action_button(ui, state, MenuAction::Quit);
}

fn show_main_menu_view(ui: &mut egui::Ui, state: &mut State) {
    action_button(ui, state, MenuAction::Search);
    action_button(ui, state, MenuAction::SearchBackward);
    ui.separator();
    action_button(ui, state, MenuAction::ToggleHtml);
    action_button(ui, state, MenuAction::DocumentInfo);
}

fn show_main_menu_help(ui: &mut egui::Ui, state: &mut State) {
    action_button(ui, state, MenuAction::Help);
}
