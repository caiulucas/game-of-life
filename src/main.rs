mod app;
mod automaton;

use app::App;
use glutin_window::{GlutinWindow, OpenGL};
use piston::{EventSettings, Events, RenderEvent, UpdateEvent, WindowSettings};

fn main() {
    let opengl = OpenGL::V4_5;

    let mut window: GlutinWindow = WindowSettings::new("spinning-square", [600, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args)
        }
    }
}
