use conrod;

use std::{
    thread,
    time::{Duration, Instant},
};

use conrod::{
    backend::glium::glium::{self, glutin, Surface},
    text::Font,
};

const WIN_W: u32 = 1024;
const WIN_H: u32 = 768;

pub struct EventLoop {
    last_update: Instant,
    ui_needs_update: bool,
}

impl EventLoop {
    pub fn new() -> Self {
        EventLoop {
            last_update: Instant::now(),
            ui_needs_update: true,
        }
    }

    pub fn next(
        &mut self,
        events_loop: &mut glium::glutin::EventsLoop,
    ) -> Vec<glium::glutin::Event> {
        let last_update = self.last_update;
        let sixteen_ms = Duration::from_millis(16);
        let duration_since_last_update = Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            thread::sleep(sixteen_ms - duration_since_last_update);
        }
        let mut events = vec![];
        events_loop.poll_events(|event| events.push(event));

        if events.is_empty() && !self.ui_needs_update {
            events_loop.run_forever(|event| {
                events.push(event);
                glutin::ControlFlow::Break
            });
        }

        self.ui_needs_update = false;
        self.last_update = Instant::now();

        events
    }

    pub fn needs_update(&mut self) {
        self.ui_needs_update = true;
    }
}

pub struct System {
    pub events_loop: glutin::EventsLoop,
    pub event_loop: EventLoop,
    pub display: glium::Display,
    pub ui: conrod::Ui,
    pub image_map: conrod::image::Map<glium::texture::Texture2d>,
    pub renderer: conrod::backend::glium::Renderer,
}

impl System {
    pub fn create(
        events_loop: glutin::EventsLoop,
        event_loop: EventLoop,
        display: glium::Display,
        ui: conrod::Ui,
        image_map: conrod::image::Map<glium::texture::Texture2d>,
        renderer: conrod::backend::glium::Renderer,
    ) -> Self {
        System {
            events_loop,
            event_loop,
            display,
            ui,
            image_map,
            renderer,
        }
    }
}

pub fn init(title: &str) -> System {
    let events_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title(title)
        .with_dimensions(glutin::dpi::LogicalSize::new(WIN_W as f64, WIN_H as f64));
    let context_builder = glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window_builder, context_builder, &events_loop).unwrap();

    let mut ui = conrod::UiBuilder::new([WIN_W as f64, WIN_H as f64]).build();
    let font_data: &[u8] = include_bytes!("../../resources/asimov.ttf");
    ui.fonts.insert(Font::from_bytes(font_data).unwrap());

    let event_loop = EventLoop::new();
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
    let renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    return System::create(events_loop, event_loop, display, ui, image_map, renderer);
}

pub fn render(system: &mut System) {
    if let Some(primitives) = system.ui.draw_if_changed() {
        system
            .renderer
            .fill(&system.display, primitives, &system.image_map);
        let mut target = system.display.draw();
        target.clear_color(1., 1., 1., 1.);
        system
            .renderer
            .draw(&system.display, &mut target, &system.image_map)
            .unwrap();
        target.finish().unwrap();
    }
}

pub fn handle_events(system: &mut System) -> bool {
    for event in system.event_loop.next(&mut system.events_loop) {
        if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &system.display) {
            system.ui.handle_event(event);
            system.event_loop.needs_update();
        }

        match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested
                | glutin::WindowEvent::KeyboardInput {
                    input:
                        glutin::KeyboardInput {
                            virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => return true,
                _ => (),
            },
            _ => (),
        }
    }
    false
}
