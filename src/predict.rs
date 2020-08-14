use super::orbit::Orbit;
use super::position::Position;

pub fn build_predict(orbit: Orbit, position: Position) -> Predict {
    let ang_vel = 2.0*std::f32::consts::PI/orbit.period;
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
    time: f32,
    mean_angular_velocity: f32,
}

impl Predict {
    pub fn predict_at(&mut self, t: f32) -> f32 {
        self.time = t;
        return self.converge(100, 0.001, self.position.theta);
    }

    fn fx(&self, e: f32, t: f32) -> f32{
        return e - self.orbit.eccentricity*e.sin() - self.mean_angular_velocity*(t - self.orbit.reference_time); 
    }

    fn fxp(&self, e: f32) -> f32 {
        return 1.0 - self.orbit.eccentricity*e.cos();
    }

    fn fxs(&self, e: f32) -> f32 {
        return self.orbit.eccentricity*e.sin();
    }

    fn converge(&self, max_step: u32, stop_epsilon: f32, x0: f32) -> f32 {
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

    fn halley_step(&self, x: f32) -> f32 {
        let denominator = 2.0*self.fxp(x).powi(2) - self.fx(x, self.time)*self.fxs(x);
        if denominator == 0.0 {
            return f32::INFINITY;
        } else {
            return x - (2.0*self.fx(x, self.time)*self.fxp(x))/denominator;
        }
    }
}
