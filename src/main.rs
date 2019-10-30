extern crate conrod;
extern crate rand;

use conrod::{
    color, position, widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget,
};

#[macro_use]
mod macros;
mod html;
mod http;
//mod main_menu_bar;
mod navigation;
mod structs;
mod support;

use structs::{State, Ids};

fn main() {
    let mut system = support::init("Microll");
    let ids = &mut Ids::new(system.ui.widget_id_generator());

    let mut state = State::default();
    *state.preloaded_pages.get_mut("microll").unwrap() = string!(include_str!("microll.html"));

    'main: loop {
        if support::handle_events(&mut system) {
            break 'main;
        }
        {
            let ui = &mut system.ui.set_widgets();
            show_main_app(ui, &mut state, ids);
        }

        support::render(&mut system);
    }
}

fn show_main_app(ui: &mut conrod::UiCell, state: &mut State, ids: &mut Ids) {
    let mut master_flowdown;
    if state.show_app_main_menu_bar {
        master_flowdown = vec![
            (
                ids.menu_bar,
                widget::Canvas::new()
                    .color(color::DARK_CHARCOAL)
                    .length(20.),
            ),
            (ids.body, widget::Canvas::new().color(color::CHARCOAL)),
        ];
    //main_menu_bar::show_app_main_menu_bar(ui, state, dimensions);
    } else {
        master_flowdown = vec![(ids.body, widget::Canvas::new().color(color::CHARCOAL))];
    }
    widget::Canvas::new()
        .flow_down(&master_flowdown)
        .set(ids.master, ui);
    show_main_app_window(ui, state, ids);
}

fn show_main_app_window(ui: &mut conrod::UiCell, state: &mut State, ids: &mut Ids) {
    if widget::Button::new()
        .parent(ids.body)
        .label("Load Webpage")
        .label_font_size(18)
        .label_y(position::Relative::Scalar(2.5))
        .top_left_of(ids.body)
        .w_h(150., 25.)
        .border(0.)
        .label_color(color::GREEN)
        .color(color::CHARCOAL)
        .set(ids.page_load_btn, ui)
        .was_clicked()
    {
        navigation::go_to_page(state);
    }
    if widget::Button::new()
        .parent(ids.body)
        .label("Load File")
        .label_font_size(18)
        .label_y(position::Relative::Scalar(2.5))
        .w_h(100., 25.)
        .border(0.)
        .label_color(color::RED)
        .color(color::CHARCOAL)
        .set(ids.file_load_btn, ui)
        .was_clicked()
    {
        state.main_body_array = html::parse_html(&state.preloaded_pages["microll"]).0;
    }
    build_webpage(ui, state, ids);
}

fn build_webpage(ui: &mut conrod::UiCell, state: &mut State, ids: &mut Ids) {
    let mut i: usize = 0;
    while i < state.main_body_array.len() {
        if state.main_body_array[i].title {
            state.window_title = state.main_body_array[i].text.clone();
        } else if state.main_body_array[i].line_break {
            ids.line_breaks.resize(ids.line_breaks.len()+1, &mut ui.widget_id_generator());
            widget::Text::new("\n")
                .parent(ids.body)
                .set(*ids.line_breaks.last().unwrap(), ui);
        } else if state.main_body_array[i].code {
            if !state.main_body_array[i - 1].code && !state.main_body_array[i + 1].code {
                ids.code_elements.resize(ids.code_elements.len()+1, &mut ui.widget_id_generator());
                widget::Text::new(&state.main_body_array[i].text).parent(ids.body).set(*ids.code_elements.last().unwrap(), ui);
            } else {
                ids.code_elements.resize(ids.code_elements.len()+1, &mut ui.widget_id_generator());
                let mut code_text: String = state.main_body_array[i].text.clone();
                while state.main_body_array[i + 1].code {
                    i += 1;
                    code_text.push_str(&state.main_body_array[i].text);
                }
                code_text.push_str("\n");
                widget::Text::new(&code_text).parent(ids.body).set(*ids.code_elements.last().unwrap(), ui);
            }
        } else if state.main_body_array[i].link {
            ids.link_elements.resize(ids.link_elements.len()+1, &mut ui.widget_id_generator());
            if widget::Button::new().label(&state.main_body_array[i].text).parent(ids.body).set(*ids.link_elements.last().unwrap(), ui).was_clicked() {
                state.url_to_get = state.main_body_array[i].url.clone();
                navigation::go_to_page(state);
            }
        } else {
            ids.text_elements.resize(ids.text_elements.len()+1, &mut ui.widget_id_generator());
            widget::Text::new(&state.main_body_array[i].text).parent(ids.body).set(*ids.text_elements.last().unwrap(), ui);
        }
        i += 1;
    }
}
/*
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
                navigation::go_to_page(state);
            }
            ui.same_line(0.);
            if ui.button(im_str!("Go!"), [50., ui.frame_height()]) {
                navigation::go_to_page(state);
            }
        });
}
*/
