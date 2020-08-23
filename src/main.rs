mod orbit;
mod position;
mod predictor;

use chrono::Utc;
use std::error::Error;
use std::vec::Vec;

use crate::orbit::{
    load_orbit_parameters_database, update_orbit_parameters_database_at, OrbitParameters,
};
use crate::position::Position;
use crate::predictor::build_predictor;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
}

impl App {
    fn render(
        &mut self,
        args: &RenderArgs,
        orbits: Vec<OrbitParameters>,
        positions: Vec<Position>,
    ) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            // Draw a box rotating around the middle of the screen.
            for i in 0..orbits.len() {
                let (rect, angle) = orbits[i].get_orbit_box(10.0);
                circle_arc(
                    GREEN,
                    1.0,
                    0.0,
                    6.28,
                    rect,
                    c.transform
                        .trans(x, y)
                        .rot_rad(angle)
                        .trans(-rect[2] / 2.0, -rect[3] / 2.0),
                    gl,
                );
                ellipse(
                    WHITE,
                    [0.0, 0.0, 7.0, 7.0],
                    c.transform
                        .trans(x - 3.5, y - 3.5)
                        .rot_rad(angle)
                        .trans(-10.0 * positions[i].x, -10.0 * positions[i].y),
                    gl,
                );
            }
        });
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let orbits_database = load_orbit_parameters_database("orbits.csv".to_string())?;
    let orbits_updated = update_orbit_parameters_database_at(orbits_database, Utc::now());

    let mut positions: Vec<Position> = vec![];
    for orb in orbits_updated.clone().into_iter() {
        let mut prd = build_predictor(orb);
        positions.push(prd.predict().clone());
        print!(
            "{planet}: {steps} steps, {epsilon}\n",
            planet = prd.get_object_name(),
            steps = prd.get_steps(),
            epsilon = prd.get_epsilon()
        );
    }

    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Ephemeris — 0.1.0", [1024, 1024])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, orbits_updated.clone(), positions.clone());
        }
    }

    Ok(())
}
