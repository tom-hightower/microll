use crate::html;
use crate::http;
use crate::structs::{State, WebpageFinder, WebpageType};
use std::fs;

fn push_back_stack(state: &mut State) {
    if let Some(current) = state.current_page.take() {
        state.back_stack.push(current);
    }
}

pub fn go_to_page(state: &mut State) {
    push_back_stack(state);
    let url = state.url_to_get.clone();
    match http::get_text(&url, state) {
        Ok((text, url)) => {
            let parsed = html::parse_html(&text);
            state.main_body_array = parsed.0;
            state.current_raw_html = text;
            state.current_page = Some(WebpageFinder::create(WebpageType::Link, url.clone()));
            add_to_history(parsed.1, url, WebpageType::Link, state);
        }
        Err(e) => println!("{}", e),
    }
    state.sub_windows.go_to_link = false;
}

pub fn go_to_file(state: &mut State) {
    push_back_stack(state);
    let contents = match fs::read_to_string(&state.file_menu.file_to_get) {
        Ok(contents) => contents,
        Err(e) => {
            println!("Could not read {}: {}", state.file_menu.file_to_get, e);
            return;
        }
    };
    let parsed = html::parse_html(&contents);
    state.main_body_array = parsed.0;
    state.current_raw_html = contents;
    state.current_page = Some(WebpageFinder::create(
        WebpageType::File,
        state.file_menu.file_to_get.clone(),
    ));
    add_to_history(
        parsed.1,
        state.file_menu.file_to_get.clone(),
        WebpageType::File,
        state,
    );
}

pub fn go_to_preload(state: &mut State, key: &str) {
    push_back_stack(state);
    let raw_html = state.preloaded_pages[key].clone();
    state.main_body_array = html::parse_html(&raw_html).0;
    state.current_raw_html = raw_html;
    state.current_page = Some(WebpageFinder::create(WebpageType::Preload, key.to_string()));
}

pub fn go_back(state: &mut State) {
    let Some(finder) = state.back_stack.pop() else {
        return;
    };
    match finder.web_type {
        WebpageType::File => {
            state.file_menu.file_to_get = finder.location.clone();
            let contents = match fs::read_to_string(&state.file_menu.file_to_get) {
                Ok(contents) => contents,
                Err(e) => {
                    println!("Could not read {}: {}", state.file_menu.file_to_get, e);
                    return;
                }
            };
            state.main_body_array = html::parse_html(&contents).0;
            state.current_raw_html = contents;
        }
        WebpageType::Link => {
            state.url_to_get = finder.location.clone();
            match http::get_text(&finder.location, state) {
                Ok((text, _url)) => {
                    state.main_body_array = html::parse_html(&text).0;
                    state.current_raw_html = text;
                }
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            }
        }
        WebpageType::Preload => {
            let raw_html = state.preloaded_pages[&finder.location].clone();
            state.main_body_array = html::parse_html(&raw_html).0;
            state.current_raw_html = raw_html;
        }
    }
    state.current_page = Some(finder);
}

pub fn save_as(state: &State) -> bool {
    match rfd::FileDialog::new()
        .add_filter("HTML", &["html", "htm"])
        .save_file()
    {
        Some(file_path) => match fs::write(&file_path, &state.current_raw_html) {
            Ok(()) => true,
            Err(e) => {
                println!("Could not save {}: {}", file_path.display(), e);
                false
            }
        },
        None => {
            println!("Save canceled");
            false
        }
    }
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

pub fn search_page(state: &mut State) {
    let query = state.search.query.to_lowercase();
    state.search.matches = if query.is_empty() {
        Vec::new()
    } else {
        state
            .main_body_array
            .iter()
            .enumerate()
            .filter(|(_, item)| item.text.to_lowercase().contains(&query))
            .map(|(i, _)| i)
            .collect()
    };
    state.search.current = 0;
}

pub fn step_search(state: &mut State, forward: bool) {
    let matches_len = state.search.matches.len();
    if matches_len == 0 {
        return;
    }
    state.search.current = if forward {
        (state.search.current + 1) % matches_len
    } else if state.search.current == 0 {
        matches_len - 1
    } else {
        state.search.current - 1
    };
}
