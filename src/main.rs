use chrono::{DateTime, Utc};

mod orbit;
mod position;
mod predictor;
mod utils;

fn main() {
    let now: DateTime<Utc> = Utc::now();
    let orbits = orbit::load_orbit_parameters_database("orbits.csv").unwrap();

    for orbit in orbits.into_iter() {
        let orbit_now = orbit.update_parameters_at(now);
        let mut predictor = predictor::build_predictor(orbit_now);
        let mut orbit_course = predictor.predict();
    }
}
