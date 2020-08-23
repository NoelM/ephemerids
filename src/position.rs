use super::orbit::OrbitCourse;
use super::orbit::OrbitParameters;

pub fn compute_position_from_orbit_course(
    orbit: OrbitParameters,
    orbit_course: OrbitCourse,
) -> Position {
    let x_peri = orbit.semi_major_axis * (orbit_course.true_anomaly.cos() - orbit.eccentricity);
    let y_peri = orbit.semi_major_axis
        * (1.0 - orbit.eccentricity.powi(2)).sqrt()
        * orbit_course.true_anomaly.sin();
    let omega = orbit.long_peri - orbit.long_asc_node;
    let cap_omega = orbit.long_asc_node;
    let inc = orbit.inclination;

    return Position {
        object_name: orbit.object_name,
        x: (omega.cos() * cap_omega.cos() - omega.sin() * cap_omega.sin() * inc.cos()) * x_peri
            + (-omega.sin() * cap_omega.cos() - omega.cos() * cap_omega.sin() * inc.cos()) * y_peri,
        y: (omega.cos() * cap_omega.sin() + omega.sin() * cap_omega.cos() * inc.cos()) * x_peri
            + (-omega.sin() * cap_omega.sin() + omega.cos() * cap_omega.cos() * inc.cos()) * y_peri,
        z: (omega.sin() * inc.sin()) * x_peri + (omega.cos() * inc.sin()) * y_peri,
    };
}

pub struct Position {
    pub object_name: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
