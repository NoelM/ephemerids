pub fn modulo_2pi(theta: f32) -> f32 {
    let two_pi = 2.0*f32::consts::PI;
    return theta - two_pi*(theta/two_pi).floor();
}
