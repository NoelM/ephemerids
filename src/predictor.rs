use super::orbit::OrbitCourse;
use crate::orbit::OrbitParameters;

pub fn build_predictor(orbit: OrbitParameters) -> Predictor {
    return Predictor {
        orbit,
        orbit_course: OrbitCourse {
            true_anomaly: 0.0,
            mean_anomaly: 0.0,
        },
    };
}

pub struct Predictor {
    orbit: OrbitParameters,
    orbit_course: OrbitCourse,
}

impl Predictor {
    pub fn predict(&mut self) -> OrbitCourse {
        self.orbit_course.mean_anomaly = self.orbit.mean_longitude - self.orbit.long_peri;
        self.orbit_course.true_anomaly = self.converge(100, 0.001);

        return self.orbit_course;
    }

    fn fx(&self, true_anomaly: f64) -> f64 {
        // The solution of the Keplerian equation
        return true_anomaly
            - self.orbit.eccentricity * true_anomaly.sin()
            - self.orbit_course.mean_anomaly;
    }

    fn fxp(&self, true_anomaly: f64) -> f64 {
        // The solution of the first derivative wrt. to the true_anomaly
        // of the Keplerian equation
        return 1.0 - self.orbit.eccentricity * true_anomaly.cos();
    }

    fn fxs(&self, true_anomaly: f64) -> f64 {
        // The solution of the second derivative wrt. to the true_anomaly
        // of the Keplerian equation
        return self.orbit.eccentricity * true_anomaly.sin();
    }

    fn converge(&self, max_step: u32, stop_epsilon: f64) -> f64 {
        let mut true_anomaly = self.orbit_course.mean_anomaly;
        let mut true_anomaly_next = 0.0;

        for _ in 0..max_step {
            true_anomaly_next = self.halley_step(true_anomaly);
            if (true_anomaly_next - true_anomaly).abs() / true_anomaly < stop_epsilon {
                break;
            }
            true_anomaly = true_anomaly_next;
        }
        return true_anomaly_next;
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
