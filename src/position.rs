use super::orbit::OrbitCourse;
use super::orbit::OrbitParameters;

pub fn get_position_from_orbit_course(
    orbit: OrbitParameters,
    orbit_course: OrbitCourse,
) -> Position {
    let rho = orbit.semi_major_axis * (1.0 - orbit.eccentricity * orbit_course.true_anomaly.cos());

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
    fn set_position_from_orbit_course(
        &mut self,
        orbit: OrbitParameters,
        orbit_course: OrbitCourse,
    ) {
        let pos = get_position_from_orbit_course(orbit, orbit_course);
        *self = pos;
    }
}
