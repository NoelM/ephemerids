struct Position {
    rho: f32,
    theta: f32,
    phi: f32,
    velocity: f32,
}

struct Orbit {
    period: f32,
    semimajor_axis: f32,
    /*
    eccentricity: f64,
    inclination: f64,
    ascending_node: f64,
    periapsis: f64,
    arg_periapsis: f64,
    */
}

fn modulo(angle: f32, limit: f32) -> f32 {
    let periods = angle/limit;
    return angle - limit*periods.floor();
}

fn compute_position(pos: Position, orbit: Orbit, secs: f32) -> Position {
    let angular_velocity = 360.0/orbit.period;

    return Position {
        rho: orbit.semimajor_axis,
        theta: modulo(pos.theta + angular_velocity*secs, 360.0),
        phi: pos.phi,
        velocity: pos.velocity,
    };
}

fn main() {
    let earth_orbit = Orbit{
        period: 365.256363004*86400.0,
        semimajor_axis: 149.6*10.0_f32.powi(6),
    };

    let earth = Position {
        rho: 149.6*10.0_f32.powi(6),
        theta: 0.0,
        phi: 90.0,
        velocity: 29.78*10.0_f32.powi(3),
    };

    let new_earth_pos = compute_position(earth, earth_orbit, 364.0*86400.0);

    println!("{angle}", angle=new_earth_pos.theta);
}
