extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    pos_x: f64,
    pos_y: f64,
    direction_x: f64,
    direction_y: f64
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);

        let (x, y) = (self.pos_x, self.pos_y);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLUE, gl);

            let transform = c.transform.trans(x, y);

            // Draw a box bouncing around the edges of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.direction_x = if self.pos_x >= 0.0 && self.pos_x <= 200.0 - 50.0 { self.direction_x } else { self.direction_x * -1.0 };
        self.direction_y = if self.pos_y >= 0.0 && self.pos_y <= 200.0 - 50.0 { self.direction_y } else { self.direction_y * -1.0 };
        self.pos_x = self.pos_x + self.direction_x * args.dt;
        self.pos_y = self.pos_y + self.direction_y * args.dt;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "bouncing-square",
            [200, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        pos_x: 25.0,
        pos_y: 25.0,
        direction_x: 20.0,
        direction_y: 10.0
    };

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}