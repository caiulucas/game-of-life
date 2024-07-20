use core::{f64, time};
use std::thread;

use glutin_window::OpenGL;
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};

use crate::automaton::{Automaton, Cell};

pub struct App {
    gl: GlGraphics,
    automaton: Automaton,
}

impl App {
    pub fn new(opengl: OpenGL) -> Self {
        Self {
            gl: GlGraphics::new(opengl),
            automaton: Automaton::default(),
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const SIZE: f64 = 6.0;

        let square = rectangle::square(0.0, 0.0, SIZE);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            let grid = self.automaton.grid;

            for i in 0..grid.len() {
                let line = grid[i];
                for j in 0..line.len() {
                    let x = i as f64 * SIZE;
                    let y = j as f64 * SIZE;
                    let transform = c.transform.trans(x, y);

                    match grid[i][j] {
                        Cell::Alive => rectangle(WHITE, square, transform, gl),
                        Cell::Dead => rectangle(BLACK, square, transform, gl),
                    }
                }
            }
        });
    }

    pub fn update(&mut self, _: &UpdateArgs) {
        self.automaton.update();
        thread::sleep(time::Duration::from_millis(50));
    }
}
