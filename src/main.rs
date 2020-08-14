mod orbit;
mod position;
mod predict;

fn main() {
    let earth_orbit = orbit::Orbit{
        period: 365.256363004*86400.0,
        semimajor_axis: 149.6*10.0_f32.powi(6),
        eccentricity: 0.01,
        reference_time: 0.0,
    };

    let earth_pos = position::Position {
        rho: 149.6*10.0_f32.powi(6),
        theta: 0.0,
    };

    let mut earth_orbit_predictor = predict::build_predict(earth_orbit, earth_pos);
    let pos = earth_orbit_predictor.predict_at(100.0);
    println!("{}", pos);
}
