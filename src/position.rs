use super::orbit::OrbitCourse;
use super::orbit::OrbitParameters;

use nalgebra::{Point3, Rotation3, Vector3};

pub fn compute_position_from_orbit_course(
    orbit: OrbitParameters,
    orbit_course: OrbitCourse,
) -> Position {
    let x_peri = orbit.semi_major_axis * (orbit_course.true_anomaly.cos() - orbit.eccentricity);
    let y_peri = orbit.semi_major_axis
        * (1.0 - orbit.eccentricity.powi(2)).sqrt()
        * orbit_course.true_anomaly.sin();
    let r_peri = Point3::new(x_peri, y_peri, 0.0);

    let rot_z_m_cap_omega = Rotation3::from_axis_angle(&Vector3::z_axis(), -orbit.long_asc_node);
    let rot_x_m_inclination = Rotation3::from_axis_angle(&Vector3::x_axis(), -orbit.inclination);
    let rot_z_m_omega =
        Rotation3::from_axis_angle(&Vector3::z_axis(), -(orbit.long_peri - orbit.long_asc_node));

    let r_ecl = rot_z_m_cap_omega * rot_x_m_inclination * rot_z_m_omega * r_peri;

    return Position {
        object_name: orbit.object_name,
        x: r_ecl.x,
        y: r_ecl.y,
        z: r_ecl.z,
    };
}

#[derive(Clone)]
pub struct Position {
    pub object_name: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
