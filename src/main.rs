use chrono::{DateTime, Utc};
use image::{Rgb, RgbImage};
use std::vec::Vec;

mod orbit;
mod position;
mod predictor;
mod utils;

fn main() {
    let now: DateTime<Utc> = Utc::now();
    let orbits = orbit::load_orbit_parameters_database("orbits.csv").unwrap();
    let mut positions: Vec<position::Position> = vec![];

    for orbit in orbits.into_iter() {
        let orbit_now = orbit.update_parameters_at(now);
        let mut predictor = predictor::build_predictor(orbit_now.clone());
        let orbit_course = predictor.predict();
        let pos = position::get_position_from_orbit_course(orbit_now, orbit_course.clone());
        positions.push(pos);
    }

    let SIZE: i32 = 256;
    let MAX_UA: i32 = 15;
    let PLANET = 3;
    let mut img = RgbImage::new(SIZE as u32, SIZE as u32);
    for position in positions.into_iter() {
        let x_red = (position.x * (SIZE as f64 / MAX_UA as f64) + SIZE as f64 / 2.0) as i32;
        let y_red = (position.y * (SIZE as f64 / MAX_UA as f64) + SIZE as f64 / 2.0) as i32;

        for i in (x_red - PLANET).max(0)..(x_red + PLANET).min(SIZE) {
            for j in (y_red - PLANET).max(0)..(y_red + PLANET).min(SIZE) {
                img.put_pixel(i as u32, j as u32, Rgb([255, 255, 255]));
            }
        }
    }

    img.save("planets.png").unwrap();
}
