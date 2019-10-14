use imgui::*;
use std::fs;

mod html;
mod http;
mod main_menu_bar;
mod structs;
mod support;

use structs::State;

fn main() {
    let mut state = State::default();
    let mut system = support::init("Microll");
    // Change capture to pass dimension as captured variable
    system.main_loop(|run, ui, dimensions| {
        show_main_app(ui, &mut state, run, dimensions);
        if state.sub_windows.go_to_link {
            show_go_url_window(ui, &mut state);
        }
        //show_test_window(ui);
    });
}

fn show_main_app(ui: &Ui, state: &mut State, _opened: &mut bool, dimensions: (u32, u32)) {
    if state.show_app_main_menu_bar {
        main_menu_bar::show_app_main_menu_bar(ui, state, dimensions);
        show_main_app_window(ui, state, dimensions);
    }
}

fn show_main_app_window(ui: &Ui, state: &mut State, dimensions: (u32, u32)) {
    Window::new(im_str!("Main"))
        .position([0.0, 15.0], Condition::Always)
        .title_bar(false)
        .resizable(false)
        .movable(false)
        .collapsible(false)
        .bring_to_front_on_focus(false)
        .size(
            [dimensions.0 as f32, (dimensions.1 as f32) - 15.0],
            Condition::Always,
        )
        .build(ui, || {
            ui.text(im_str!("Current frame dimensions: {:?}", dimensions));
            ui.text(im_str!("Press the green square to pull sample html:"));
            if ColorButton::new(im_str!("Green color"), [0.0, 1.0, 0.0, 1.0])
                .size([50.0, 50.0])
                .build(ui)
            {
                go_to_page(state);
            }
            ui.same_line(0.);
            if ColorButton::new(im_str!("Red color"), [1.0, 0.0, 0.0, 1.0])
                .size([50.0, 50.0])
                .build(ui)
            {
                go_to_file(state);
            }
            //let mut print_str = String::new();
            let mut i: usize = 0;
            while i < state.main_body_array.len() {
                if state.main_body_array[i].title {
                    state.window_title = state.main_body_array[i].text.clone();
                } else if state.main_body_array[i].line_break {
                    ui.new_line();
                } else if state.main_body_array[i].code {
                    ui.text_wrapped(&im_str!("\t{}", &state.main_body_array[i].text));
                } else if state.main_body_array[i].link {
                    if ui.button(
                        &im_str!("{}", &state.main_body_array[i].text),
                        ui.calc_text_size(
                            &im_str!("{:}", state.main_body_array[i].text),
                            false,
                            0.,
                        ),
                    ) {
                        state.url_to_get = im_str!("{}", state.main_body_array[i].url);
                        go_to_page(state);
                    }
                } else {
                    ui.text_wrapped(&im_str!("{}", &state.main_body_array[i].text));
                }
                i += 1;
            }
        });
}

fn show_go_url_window(ui: &Ui, state: &mut State) {
    Window::new(im_str!("Go To URL..."))
        .size([0.0, 0.0], Condition::FirstUseEver)
        .always_auto_resize(true)
        .build(ui, || {
            if ui
                .input_text(im_str!(""), &mut state.url_to_get)
                .enter_returns_true(true)
                .build()
            {
                go_to_page(state);
            }
            ui.same_line(0.);
            if ui.button(im_str!("Go!"), [50., ui.frame_height()]) {
                go_to_page(state);
            }
        });
}

fn go_to_page(state: &mut State) {
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

fn go_to_file(state: &mut State) {
    let contents = fs::read_to_string(&state.file_menu.file_to_get).expect("Something went wrong reading the file");
    state.main_body_array = html::parse_html(&contents);
}