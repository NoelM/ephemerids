use super::orbit::Orbit;
use super::orbit::OrbitCourse;

pub fn build_predictor(orbit: Orbit) -> Predictor {
    return Predictor {
        orbit,
        orbit_course: OrbitCourse {
            true_anomaly: 0.0,
            mean_anomaly: 0.0,
        },
        time: orbit.reference_time,
        mean_angular_velocity: 2.0 * std::f64::consts::PI / orbit.period,
    };
}

pub struct Predictor {
    orbit: Orbit,
    orbit_course: OrbitCourse,
    time: f64,
    mean_angular_velocity: f64,
}

impl Predictor {
    pub fn predict_at(&mut self, t: f64) -> OrbitCourse {
        self.time = t;
        self.orbit_course.mean_anomaly =
            self.mean_angular_velocity * (self.time - self.orbit.reference_time);

        self.orbit_course.true_anomaly = self.converge(100, 0.001, self.orbit_course.mean_anomaly);
        return self.orbit_course;
    }

    fn fx(&self, mean_anomaly: f64) -> f64 {
        /// The solution of the Keplerian equation
        return mean_anomaly
            - self.orbit.eccentricity * mean_anomaly.sin()
            - self.orbit_course.mean_anomaly;
    }

    fn fxp(&self, mean_anomaly: f64) -> f64 {
        /// The solution of the first derivative wrt. to the mean_anomaly
        /// of the Keplerian equation
        return 1.0 - self.orbit.eccentricity * mean_anomaly.cos();
    }

    fn fxs(&self, mean_anomaly: f64) -> f64 {
        /// The solution of the second derivative wrt. to the mean_anomaly
        /// of the Keplerian equation
        return self.orbit.eccentricity * mean_anomaly.sin();
    }

    fn converge(&self, max_step: u32, stop_epsilon: f64, init_mean_anomaly: f64) -> f64 {
        let mut mean_anomaly = init_mean_anomaly;
        let mut mean_anomaly_next = 0.0;

        for _ in 0..max_step {
            mean_anomaly_next = self.halley_step(mean_anomaly);
            if (mean_anomaly_next - mean_anomaly).abs() / mean_anomaly < stop_epsilon {
                break;
            }
            mean_anomaly = mean_anomaly_next;
        }
        return mean_anomaly_next;
    }

    fn halley_step(&self, x: f64) -> f64 {
        let denominator = 2.0 * self.fxp(x).powi(2) - self.fx(x) * self.fxs(x);
        return if denominator == 0.0 {
            f64::INFINITY
        } else {
            x - (2.0 * self.fx(x) * self.fxp(x)) / denominator
        };
    }
}
