use crate::html;
use crate::http;
use crate::structs::{State, WebpageFinder, WebpageType};
use std::fs;

pub fn go_to_page(state: &mut State) {
    let url = state.url_to_get.clone();
    match http::get_text(&url, state) {
        Ok((text, url)) => {
            let parsed = html::parse_html(&text);
            state.main_body_array = parsed.0;
            add_to_history(parsed.1, url, WebpageType::Link, state);
        }
        Err(e) => println!("{}", e),
    }
    state.sub_windows.go_to_link = false;
}

pub fn go_to_file(state: &mut State) {
    let contents = match fs::read_to_string(&state.file_menu.file_to_get) {
        Ok(contents) => contents,
        Err(e) => {
            println!("Could not read {}: {}", state.file_menu.file_to_get, e);
            return;
        }
    };
    let parsed = html::parse_html(&contents);
    state.main_body_array = parsed.0;
    add_to_history(
        parsed.1,
        state.file_menu.file_to_get.clone(),
        WebpageType::File,
        state,
    );
}

pub fn file_picker(state: &mut State) -> bool {
    match rfd::FileDialog::new()
        .add_filter("HTML", &["html", "htm"])
        .pick_file()
    {
        Some(file_path) => {
            state.file_menu.file_to_get = file_path.to_string_lossy().into_owned();
            true
        }
        None => {
            println!("File pick canceled");
            false
        }
    }
}

pub fn add_to_history(title: String, url: String, web_type: WebpageType, state: &mut State) {
    state.history.remove(&title);
    let finder: WebpageFinder = WebpageFinder::create(web_type, url);
    state.history.insert(title, finder);
}
