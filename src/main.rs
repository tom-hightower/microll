use imgui::*;

mod support;

fn main() {
    let system = support::init(file!());
    system.main_loop(|_, ui| {
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