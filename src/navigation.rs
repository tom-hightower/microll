use crate::html;
use crate::http;
use crate::structs::{State, WebpageFinder, WebpageType};
use std::fs;

extern crate nfd;
use nfd::Response;

pub fn go_to_page(state: &mut State) {
    let html_text;
    match http::get_text(&state.url_to_get.to_owned(), state) {
        Ok((text, url)) => {
            html_text = text;
            let parsed = html::parse_html(&html_text);
            state.main_body_array = parsed.0;
            add_to_history(parsed.1, url, WebpageType::Link, state);
        }
        Err(e) => println!("{}", e),
    }
    state.sub_windows.go_to_link = false;
}

pub fn go_to_file(state: &mut State) {
    let contents = fs::read_to_string(&state.file_menu.file_to_get)
        .expect("Something went wrong reading the file");
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
    let result = nfd::open_file_dialog(Some("html"), None).unwrap_or_else(|e| {
        panic!(e);
    });
    return match result {
        Response::Okay(file_path) => {
            state.file_menu.file_to_get = file_path;
            true
        }
        _ => {
            println!("File pick canceled");
            false
        }
    };
}

pub fn add_to_history(title: String, url: String, web_type: WebpageType, state: &mut State) {
    if state.history.contains_key(&title) {
        state.history.remove(&title);
    }
    let finder: WebpageFinder = WebpageFinder::create(web_type, url);
    state.history.insert(title, finder);
}
