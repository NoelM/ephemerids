#[derive(Copy, Clone)]
pub struct Orbit {
    pub period: f64,
    pub semimajor_axis: f64,
    pub eccentricity: f64,
    pub reference_time: f64,
    /*
    inclination: f64,
    ascending_node: f64,
    periapsis: f64,
    arg_periapsis: f64,
    */
}

#[derive(Copy, Clone)]
pub struct OrbitCourse {
    pub true_anomaly: f64,
    pub mean_anomaly: f64,
}
