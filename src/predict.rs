use super::orbit::Orbit;
use super::position::Position;

pub fn build_predict(orbit: Orbit, position: Position) -> Predict {
    let ang_vel = 2.0*std::f64::consts::PI/orbit.period;
    return Predict {
        orbit: orbit,
        position: position,
        time: 0.0,
        mean_angular_velocity: ang_vel,
    }
}

pub struct Predict {
    orbit: Orbit,
    position: Position,
    time: f64,
    mean_angular_velocity: f64,
}

impl Predict {
    pub fn predict_at(&mut self, t: f64) -> f64 {
        self.time = t;
        return self.converge(100, 0.001, self.position.theta);
    }

    fn fx(&self, e: f64, t: f64) -> f64 {
        return e - self.orbit.eccentricity*e.sin() - self.mean_angular_velocity*(t - self.orbit.reference_time); 
    }

    fn fxp(&self, e: f64) -> f64 {
        return 1.0 - self.orbit.eccentricity*e.cos();
    }

    fn fxs(&self, e: f64) -> f64 {
        return self.orbit.eccentricity*e.sin();
    }

    fn converge(&self, max_step: u32, stop_epsilon: f64, x0: f64) -> f64 {
        let mut x = x0;
        let mut xnext = 0.0;

        for _ in 0..max_step {
            xnext = self.halley_step(x);
            if (xnext - x).abs()/x < stop_epsilon {
                break;
            }
            x = xnext;
        }
        return xnext;
    }

    fn halley_step(&self, x: f64) -> f64 {
        let denominator = 2.0*self.fxp(x).powi(2) - self.fx(x, self.time)*self.fxs(x);
        if denominator == 0.0 {
            return f64::INFINITY;
        } else {
            return x - (2.0*self.fx(x, self.time)*self.fxp(x))/denominator;
        }
    }
}
