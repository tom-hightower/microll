mod html;
mod http;
mod main_menu_bar;
mod navigation;
mod structs;

use eframe::egui;
use structs::State;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Microll",
        options,
        Box::new(|cc| Ok(Box::new(MicrollApp::new(cc)))),
    )
}

struct MicrollApp {
    state: State,
}

impl MicrollApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "asimov".to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
                "../resources/asimov.ttf"
            ))),
        );
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "asimov".to_owned());
        fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(0, "asimov".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        let mut visuals = egui::Visuals::light();
        visuals.panel_fill = egui::Color32::WHITE;
        visuals.window_fill = egui::Color32::WHITE;
        cc.egui_ctx.set_visuals(visuals);

        MicrollApp {
            state: State::default(),
        }
    }
}

impl eframe::App for MicrollApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let state = &mut self.state;
        egui::Panel::top("menu_bar").show(ui, |ui| {
            main_menu_bar::show_app_main_menu_bar(ui, state);
        });
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, |ui| {
                    show_main_content(ui, state);
                });
        });
        if state.sub_windows.go_to_link {
            show_go_url_window(ui.ctx(), state);
        }
        if state.sub_windows.search {
            show_search_window(ui.ctx(), state);
        }
        if state.sub_windows.document_info {
            show_document_info_window(ui.ctx(), state);
        }
        if state.sub_windows.help {
            show_help_window(ui.ctx(), state);
        }
    }
}

fn show_main_content(ui: &mut egui::Ui, state: &mut State) {
    ui.label("Press the green square to pull sample html:");
    ui.horizontal(|ui| {
        if ui
            .add(
                egui::Button::new("")
                    .fill(egui::Color32::GREEN)
                    .min_size(egui::vec2(50.0, 50.0)),
            )
            .clicked()
        {
            navigation::go_to_page(state);
        }
        if ui
            .add(
                egui::Button::new("")
                    .fill(egui::Color32::RED)
                    .min_size(egui::vec2(50.0, 50.0)),
            )
            .clicked()
        {
            navigation::go_to_preload(state, "microll");
        }
    });
    if state.sub_windows.show_raw_html {
        ui.monospace(state.current_raw_html.as_str());
    } else {
        build_webpage(ui, state);
    }
}

enum Inline {
    Text(usize, String),
    Code(usize, String),
    Link { index: usize, text: String, url: String },
}

fn highlighted_text(text: String, index: usize, highlight: Option<usize>) -> egui::RichText {
    let rich = egui::RichText::new(text);
    if Some(index) == highlight {
        rich.background_color(egui::Color32::YELLOW)
    } else {
        rich
    }
}

fn flush_inline(
    ui: &mut egui::Ui,
    run: &mut Vec<Inline>,
    clicked_url: &mut Option<String>,
    highlight: Option<usize>,
) {
    if run.is_empty() {
        return;
    }
    ui.horizontal_wrapped(|ui| {
        for item in run.drain(..) {
            match item {
                Inline::Text(index, text) => {
                    let response = ui.label(highlighted_text(text, index, highlight));
                    if Some(index) == highlight {
                        response.scroll_to_me(Some(egui::Align::Center));
                    }
                }
                Inline::Code(index, text) => {
                    let response = ui.code(highlighted_text(text, index, highlight));
                    if Some(index) == highlight {
                        response.scroll_to_me(Some(egui::Align::Center));
                    }
                }
                Inline::Link { index, text, url } => {
                    let response = ui.link(highlighted_text(text, index, highlight));
                    if response.clicked() {
                        *clicked_url = Some(url);
                    }
                    if Some(index) == highlight {
                        response.scroll_to_me(Some(egui::Align::Center));
                    }
                }
            }
        }
    });
}

fn build_webpage(ui: &mut egui::Ui, state: &mut State) {
    let highlight = if state.sub_windows.search && !state.search.matches.is_empty() {
        Some(state.search.matches[state.search.current])
    } else {
        None
    };
    let mut run: Vec<Inline> = Vec::new();
    let mut clicked_url: Option<String> = None;
    let mut new_title: Option<String> = None;
    let len = state.main_body_array.len();
    let mut i: usize = 0;
    while i < len {
        let item = &state.main_body_array[i];
        if item.title {
            new_title = Some(item.text.clone());
        } else if item.line_break {
            flush_inline(ui, &mut run, &mut clicked_url, highlight);
            ui.add_space(ui.text_style_height(&egui::TextStyle::Body));
        } else if item.code {
            let prev_code = i > 0 && state.main_body_array[i - 1].code;
            let next_code = i + 1 < len && state.main_body_array[i + 1].code;
            if !prev_code && !next_code {
                run.push(Inline::Code(i, item.text.clone()));
            } else {
                // Consecutive code items form one block: concatenate them.
                let start_i = i;
                let mut code_text = item.text.clone();
                while i + 1 < len && state.main_body_array[i + 1].code {
                    i += 1;
                    code_text.push_str(&state.main_body_array[i].text);
                }
                flush_inline(ui, &mut run, &mut clicked_url, highlight);
                let block_highlighted = highlight.is_some_and(|h| (start_i..=i).contains(&h));
                let rich = egui::RichText::new(code_text);
                let rich = if block_highlighted {
                    rich.background_color(egui::Color32::YELLOW)
                } else {
                    rich
                };
                let response = ui.monospace(rich);
                if block_highlighted {
                    response.scroll_to_me(Some(egui::Align::Center));
                }
            }
        } else if item.link {
            run.push(Inline::Link {
                index: i,
                text: item.text.clone(),
                url: item.url.clone(),
            });
        } else {
            run.push(Inline::Text(i, item.text.clone()));
        }
        i += 1;
    }
    flush_inline(ui, &mut run, &mut clicked_url, highlight);

    if let Some(title) = new_title {
        state.window_title = title;
    }
    if let Some(url) = clicked_url {
        state.url_to_get = url;
        navigation::go_to_page(state);
    }
}

fn show_go_url_window(ctx: &egui::Context, state: &mut State) {
    let mut open = true;
    egui::Window::new("Go To URL...")
        .open(&mut open)
        .resizable(false)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                let response = ui.add(
                    egui::TextEdit::singleline(&mut state.url_to_get).desired_width(400.0),
                );
                let entered =
                    response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                if entered || ui.button("Go!").clicked() {
                    navigation::go_to_page(state);
                }
            });
        });
    if !open {
        state.sub_windows.go_to_link = false;
    }
}

fn show_search_window(ctx: &egui::Context, state: &mut State) {
    let mut open = true;
    egui::Window::new("Search")
        .open(&mut open)
        .resizable(false)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                let response = ui
                    .add(egui::TextEdit::singleline(&mut state.search.query).desired_width(300.0));
                if response.changed() {
                    navigation::search_page(state);
                }
                if ui.button("Previous").clicked() {
                    navigation::step_search(state, false);
                }
                if ui.button("Next").clicked() {
                    navigation::step_search(state, true);
                }
            });
            if state.search.matches.is_empty() {
                ui.label("No matches");
            } else {
                ui.label(format!(
                    "Match {} of {}",
                    state.search.current + 1,
                    state.search.matches.len()
                ));
            }
        });
    if !open {
        state.sub_windows.search = false;
    }
}

fn show_document_info_window(ctx: &egui::Context, state: &mut State) {
    let mut open = true;
    egui::Window::new("Document Info")
        .open(&mut open)
        .resizable(false)
        .show(ctx, |ui| {
            ui.label(format!("Title: {}", state.window_title));
            let location = state
                .current_page
                .as_ref()
                .map(|finder| finder.location.clone())
                .unwrap_or_else(|| String::from("(none)"));
            ui.label(format!("Location: {location}"));
            let link_count = state.main_body_array.iter().filter(|i| i.link).count();
            ui.label(format!("Links: {link_count}"));
            let word_count: usize = state
                .main_body_array
                .iter()
                .map(|i| i.text.split_whitespace().count())
                .sum();
            ui.label(format!("Words: {word_count}"));
            ui.label(format!("HTML size: {} bytes", state.current_raw_html.len()));
        });
    if !open {
        state.sub_windows.document_info = false;
    }
}

fn show_help_window(ctx: &egui::Context, state: &mut State) {
    let mut open = true;
    egui::Window::new("Help")
        .open(&mut open)
        .resizable(false)
        .show(ctx, |ui| {
            ui.label(format!("Microll v{}", env!("CARGO_PKG_VERSION")));
            ui.label("A text-based web browser.");
            ui.separator();
            ui.label("Keyboard shortcuts:");
            ui.label("Ctrl+G    Go to URL");
            ui.label("Alt+Left  Go Back");
            ui.label("Ctrl+O    Open file");
            ui.label("Alt+F4    Quit");
            ui.label("/         Search");
            ui.label("?         Search backward");
            ui.label("\\         Toggle HTML");
            ui.label("=         Document Info");
        });
    if !open {
        state.sub_windows.help = false;
    }
}
