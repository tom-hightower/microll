use imgui::*;

mod html;
mod http;
mod main_menu_bar;
mod structs;
mod support;

use structs::State;

fn main() {
    let mut state = State::default();
    let mut system = support::init(file!());
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
        main_menu_bar::show_app_main_menu_bar(ui, state);
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
                let html_text = http::get_text(&String::from(state.url_to_get.to_str().to_owned())).unwrap();
                let parser = html::parse_html(&html_text);
                state.main_body_array = html::traverse_document(parser);
            }
            for x in state.main_body_array.iter() {
                ui.text_wrapped(&im_str!("{}", x));
            }
        });
}

fn show_go_url_window(ui: &Ui, state: &mut State) {
    Window::new(im_str!("Go To URL..."))
        .size([0.0, 0.0], Condition::FirstUseEver)
        .always_auto_resize(true)
        .build(ui, || {
            ui.input_text(im_str!("Go To URL"), &mut state.url_to_get)
                .enter_returns_true(true)
                .build();
        });
}
