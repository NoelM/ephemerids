use super::orbit::Orbit;
use super::orbit::OrbitCourse;

pub fn get_position_from_orbit_course(orbit: Orbit, orbit_course: OrbitCourse) -> Position {
    let rho = orbit.semimajor_axis * (1.0 - orbit.eccentricity * orbit_course.true_anomaly.cos());

    let cos_theta = (orbit_course.true_anomaly.cos() - orbit.eccentricity)
        / (1.0 - orbit.eccentricity * orbit_course.true_anomaly.cos());
    let theta = cos_theta.acos() * orbit_course.true_anomaly.sin().signum();

    return Position {
        rho,
        theta,
        phi: 0.0,
    };
}

pub struct Position {
    pub rho: f64,
    pub theta: f64,
    pub phi: f64,
}

impl Position {
    fn set_position_from_orbit_course(&mut self, orbit: Orbit, orbit_course: OrbitCourse) {
        let pos = get_position_from_orbit_course(orbit, orbit_course);
        *self = pos;
    }
}
