use crate::orbit::OrbitCourse;
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
    let mut orbits_now: Vec<orbit::OrbitParameters> = vec![];

    for orbit in orbits.into_iter() {
        let orbit_now = orbit.update_parameters_at(now);
        orbits_now.push(orbit_now.clone());
        let mut predictor = predictor::build_predictor(orbit_now.clone());
        let orbit_course = predictor.predict();
        let pos = position::get_position_from_orbit_course(orbit_now, orbit_course.clone());
        positions.push(pos);
    }

    let SIZE: i32 = 1024;
    let MAX_UA: i32 = 5;
    let PLANET = 3;
    let mut img = RgbImage::new(SIZE as u32, SIZE as u32);

    for orbit in orbits_now.into_iter() {
        for true_anomaly in 0..360 {
            let course = orbit::OrbitCourse {
                object_name: "null".to_string(),
                true_anomaly: true_anomaly as f64 / 360.0 * 2.0 * std::f64::consts::PI,
                mean_anomaly: 0.0,
            };
            let position = position::get_position_from_orbit_course(orbit.clone(), course);
            let x_red = (position.x * (SIZE as f64 / MAX_UA as f64) + SIZE as f64 / 2.0) as u32;
            let y_red = (position.y * (SIZE as f64 / MAX_UA as f64) + SIZE as f64 / 2.0) as u32;
            if (x_red >= 0 && x_red < SIZE as u32) && (y_red >= 0 && y_red < SIZE as u32) {
                img.put_pixel(x_red, y_red, Rgb([0, 255, 0]));
            }
        }
    }

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
