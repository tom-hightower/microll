pub struct State {
    pub show_app_main_menu_bar: bool,
    pub file_menu: FileMenuState,
}

impl Default for State {
    fn default() -> Self {
        State {
            show_app_main_menu_bar: true,
            file_menu: Default::default(),
        }
    }
}

pub struct FileMenuState {
    pub test_enabled: bool,
    pub can_search: bool,
}

impl Default for FileMenuState {
    fn default() -> Self {
        FileMenuState {
            test_enabled: true,
            can_search: true,
        }
    }
}