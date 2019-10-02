use imgui::ImString;

pub struct State {
    pub show_app_main_menu_bar: bool,
    pub file_menu: FileMenuState,
    pub url_to_get: ImString,
    pub main_body_array: Vec<Vec<u8>>,
    pub sub_windows: SubWindowVisibility,
}

impl Default for State {
    fn default() -> Self {
        State {
            show_app_main_menu_bar: true,
            file_menu: Default::default(),
            url_to_get: ImString::new("https://adriann.github.io/rust_parser.html"),
            main_body_array: vec![String::from("Test").as_bytes().to_vec()],
            sub_windows: Default::default(),
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

pub struct SubWindowVisibility {
    pub go_to_link: bool,
}

impl Default for SubWindowVisibility {
    fn default() -> Self {
        SubWindowVisibility {
            go_to_link: false,
        }
    }
}