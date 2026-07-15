use eframe::egui;
use eframe::egui::{Key, KeyboardShortcut, Modifiers};

use crate::navigation;
use crate::structs::State;

/// Renders as e.g. "Ctrl+G" or "/"
pub fn format_shortcut(shortcut: &KeyboardShortcut) -> String {
    let mut s = String::new();
    if shortcut.modifiers.ctrl {
        s.push_str("Ctrl+");
    }
    if shortcut.modifiers.alt {
        s.push_str("Alt+");
    }
    if shortcut.modifiers.shift {
        s.push_str("Shift+");
    }
    let key = shortcut.logical_key;
    let key_text = match key {
        Key::ArrowLeft | Key::ArrowRight | Key::ArrowUp | Key::ArrowDown => key.name(),
        _ => key.symbol_or_name(),
    };
    s.push_str(key_text);
    s
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MenuAction {
    GoToUrl,
    GoBack,
    OpenFile,
    Quit,
    SearchBackward,
    Search,
    ToggleHtml,
    DocumentInfo,
    Help,
}

/// Menu/help display order.
pub const ALL: &[MenuAction] = &[
    MenuAction::GoToUrl,
    MenuAction::GoBack,
    MenuAction::OpenFile,
    MenuAction::Quit,
    MenuAction::Search,
    MenuAction::SearchBackward,
    MenuAction::ToggleHtml,
    MenuAction::DocumentInfo,
    MenuAction::Help,
];

impl MenuAction {
    pub fn label(self) -> &'static str {
        match self {
            MenuAction::GoToUrl => "Go to URL",
            MenuAction::GoBack => "Go Back",
            MenuAction::OpenFile => "Open file",
            MenuAction::Quit => "Quit",
            MenuAction::SearchBackward => "Search backward",
            MenuAction::Search => "Search",
            MenuAction::ToggleHtml => "Toggle HTML",
            MenuAction::DocumentInfo => "Document Info",
            MenuAction::Help => "Help",
        }
    }

    pub fn shortcut(self) -> Option<KeyboardShortcut> {
        match self {
            MenuAction::GoToUrl => Some(KeyboardShortcut::new(Modifiers::CTRL, Key::G)),
            MenuAction::GoBack => Some(KeyboardShortcut::new(Modifiers::ALT, Key::ArrowLeft)),
            MenuAction::OpenFile => Some(KeyboardShortcut::new(Modifiers::CTRL, Key::O)),
            MenuAction::Quit => Some(KeyboardShortcut::new(Modifiers::ALT, Key::F4)),
            MenuAction::SearchBackward => {
                Some(KeyboardShortcut::new(Modifiers::NONE, Key::Questionmark))
            }
            MenuAction::Search => Some(KeyboardShortcut::new(Modifiers::NONE, Key::Slash)),
            MenuAction::ToggleHtml => Some(KeyboardShortcut::new(Modifiers::NONE, Key::Backslash)),
            MenuAction::DocumentInfo => Some(KeyboardShortcut::new(Modifiers::NONE, Key::Equals)),
            MenuAction::Help => Some(KeyboardShortcut::new(Modifiers::NONE, Key::F1)),
        }
    }

    pub fn enabled(self, state: &State) -> bool {
        match self {
            MenuAction::GoBack => !state.back_stack.is_empty(),
            MenuAction::Search | MenuAction::SearchBackward => state.file_menu.can_search,
            _ => true,
        }
    }

    pub fn apply(self, state: &mut State, ctx: &egui::Context) {
        match self {
            MenuAction::GoToUrl => state.sub_windows.go_to_link = !state.sub_windows.go_to_link,
            MenuAction::GoBack => navigation::go_back(state),
            MenuAction::OpenFile => {
                if navigation::file_picker(state) {
                    navigation::go_to_file(state);
                }
            }
            MenuAction::Quit => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
            MenuAction::SearchBackward => {
                state.sub_windows.search = true;
                navigation::step_search(state, false);
            }
            MenuAction::Search => state.sub_windows.search = true,
            MenuAction::ToggleHtml => {
                state.sub_windows.show_raw_html = !state.sub_windows.show_raw_html;
            }
            MenuAction::DocumentInfo => state.sub_windows.document_info = true,
            MenuAction::Help => state.sub_windows.help = true,
        }
    }
}
