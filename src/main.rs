mod orbit;
mod position;
mod predictor;
mod projector;

use chrono::Utc;
use std::error::Error;
use std::vec::Vec;

use crate::orbit::{
    load_orbit_parameters_database, update_orbit_parameters_database_at, OrbitParameters,
};
use crate::position::Position;
use crate::predictor::build_predictor;

use clap::{App, Arg};
use std::collections::HashMap;
use crate::projector::translate_to_earth;

/*
use glutin_window::GlutinWindow as Window;
use nalgebra::{Point2, Rotation2};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent};
use piston::window::WindowSettings;

pub struct WindowApp {
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
        const MULT: f64 = 30.0;

        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            // Draw a box rotating around the middle of the screen.
            for i in 0..orbits.len() {
                let (rect, angle) = orbits[i].get_orbit_box(MULT);
                circle_arc(
                    GREEN,
                    1.0,
                    0.0,
                    6.28,
                    rect,
                    c.transform.trans(x, y).rot_rad(angle),
                    gl,
                );
                ellipse(
                    WHITE,
                    [0.0, 0.0, 7.0, 7.0],
                    c.transform
                        .trans(x, y)
                        .trans(-MULT * positions[i].x - 3.5, -MULT * positions[i].y - 3.5),
                    gl,
                );
            }
        });
    }
}*/

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("ephemeris")
        .version("0.2.0")
        .about("Computes stars and planets ephemeris")
        .author("Noël Martin")
        .arg(Arg::new("orbits-db")
            .short('d')
            .long("orbits-db")
            .value_name("FILE")
            .about("Planets orbits DB file")
            .takes_value(true)
            .required(true)
        )
        .get_matches();

    let orbit_db_filename = matches.value_of("orbits-db").unwrap();
    let orbits_params =
        load_orbit_parameters_database(orbit_db_filename.to_string())?;

    let updated_orbits_params =
        update_orbit_parameters_database_at(orbits_params, Utc::now());

    let mut position_map: HashMap<String, Position> = HashMap::new();
    for orbit in updated_orbits_params.clone().into_iter() {
        let mut predictor = build_predictor(orbit);
        let position = predictor.predict();
        position_map.insert(position.object_name.clone(), position.clone());
        print!(
            "{planet}: {steps} steps, {epsilon}\n",
            planet = predictor.get_object_name(),
            steps = predictor.get_steps(),
            epsilon = predictor.get_epsilon()
        )
    }

    for elem in position_map.clone().into_iter() {
        print!(
            "{planet}: ({x}, {y}, {z})\n",
            planet = elem.0,
            x = elem.1.x,
            y = elem.1.y,
            z = elem.1.z,
        )
    }

    let from_earth_position = translate_to_earth(position_map);

    print!("From Earth \n\n");
    for elem in from_earth_position.clone().into_iter() {
        print!(
            "{planet}: ({x}, {y}, {z})\n",
            planet = elem.0,
            x = elem.1.x,
            y = elem.1.y,
            z = elem.1.z,
        )
    }
    /*let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Ephemeris — 0.1.0", [1024, 1024])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = WindowApp {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, orbits_updated.clone(), positions.clone());
        }
    }*/

    Ok(())
}
