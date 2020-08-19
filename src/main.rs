use crate::position::get_position_from_orbit_course;

mod orbit;
mod position;
mod predictor;

fn main() {
    let earth_orbit = orbit::Orbit {
        period: 365.256363004 * 86400.0,
        semimajor_axis: 149.6 * 10.0_f64.powi(6),
        eccentricity: 0.01,
        reference_time: 0.0,
    };

    let mut earth_orbit_predictor = predictor::build_predictor(earth_orbit);

    // Predict position at 1 day after the first one
    let orbit_course = earth_orbit_predictor.predict_at(86400.0);
    let position = get_position_from_orbit_course(earth_orbit, orbit_course);
    print!(
        "r = {rho} km\nt = {theta} rad",
        rho = position.rho,
        theta = position.theta
    );
}
