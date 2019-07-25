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
    ui.window(im_str!("Main"))
        .position([0.0, 15.0], Condition::Always)
        .title_bar(false)
        .resizable(false)
        .movable(false)
        .collapsible(false)
        .no_bring_to_front_on_focus(true)
        .size(
            [dimensions.0 as f32, (dimensions.1 as f32) - 15.0],
            Condition::Always,
        )
        .build(|| {
            ui.text(im_str!("Current frame dimensions: {:?}", dimensions));
            ui.text_wrapped(&im_str!("{:?}", state.main_body_text));
            ui.text(im_str!("Press the green square to pull sample html:"));
            if ui
                .color_button(im_str!("Green color"), [0.0, 1.0, 0.0, 1.0])
                .size([100.0, 50.0])
                .build()
            {
                let html_text = http::get_text(&state.url_to_get).unwrap();
                let parser = html::parse_html(&html_text);
                state.main_body_text = html::get_elements(parser, "p")[0].clone();
                println!("{:?}", state.main_body_text);
            }
        });
}

fn show_test_window(ui: &Ui) {
    ui.window(im_str!("Microll"))
        .size([0.0, 0.0], Condition::FirstUseEver)
        .build(|| {
            ui.text(im_str!("Hello, world!"));
            ui.text(im_str!("Example program:"));
            ui.text(im_str!("Microll by Tom Hightower"));
            ui.separator();
            let mouse_pos = ui.io().mouse_pos;
            ui.text(format!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos[0], mouse_pos[1]
            ));
        });
}
