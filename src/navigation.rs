use std::fs;
use crate::html;
use crate::http;
use crate::structs::State;

pub fn go_to_page(state: &mut State) {
    let html_text;
    match http::get_text(&String::from(state.url_to_get.to_str().to_owned())) {
        Ok(text) => {
            html_text = text;
            state.main_body_array = html::parse_html(&html_text);
        }
        Err(e) => println!("{}", e)
    }
    state.sub_windows.go_to_link = false;
}

pub fn go_to_file(state: &mut State) {
    let contents = fs::read_to_string(&state.file_menu.file_to_get).expect("Something went wrong reading the file");
    state.main_body_array = html::parse_html(&contents);
}