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
        let mut predictor = predictor::build_predictor(orbit_now.clone());
        let orbit_course = predictor.predict();
        let pos = position::get_position_from_orbit_course(orbit_now, orbit_course.clone());
        print!(
            "Object: {object_name}\nR = {rho} UA\nTheta: {theta}\n\n",
            object_name = orbit_course.object_name,
            rho = pos.rho,
            theta = pos.theta
        );
    }
}
