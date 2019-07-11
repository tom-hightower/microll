use imgui::*;

mod support;
mod main_menu_bar;
mod structs;
use structs::State;

fn main() {
    let mut state = State::default();
    let system = support::init(file!());
    system.main_loop(|run, ui| {
        show_main_app(ui, &mut state, run);
        ui.window(im_str!("Microll"))
            .size([300.0, 100.0], Condition::FirstUseEver)
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
    });
}

fn show_main_app(ui: &Ui, state: &mut State, _opened: &mut bool) {
    if state.show_app_main_menu_bar {
        main_menu_bar::show_app_main_menu_bar(ui, state)
    }
}
