pub fn modulo_2pi(theta: f64) -> f32 {
    let two_pi = 2.0*f64::consts::PI;
    return theta - two_pi*(theta/two_pi).floor();
}
