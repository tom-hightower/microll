use imgui::*;

mod http;
mod main_menu_bar;
mod structs;
mod support;

use structs::State;

fn main() {
    let mut state = State::default();
    let system = support::init(file!());
    let dimensions: (u32, u32) = system.display.get_framebuffer_dimensions();
    system.main_loop(|run, ui| {
        show_main_app(ui, &mut state, run, dimensions);
        show_test_window(ui);
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
            ui.text(im_str!("{:?}", state.main_body_text));
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
