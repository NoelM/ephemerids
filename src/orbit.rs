use std::error::Error;
use std::fs::File;
use std::vec::Vec;

use chrono::{DateTime, Duration, Utc};
use csv;
use serde::Deserialize;

use crate::utils::modulo_2pi;

#[derive(Debug, Deserialize, Clone)]
pub struct OrbitParameters {
    pub object_name: String,
    pub semi_major_axis: f64,
    pub eccentricity: f64,
    #[serde(with = "deg_to_rad")]
    pub inclination: f64,
    #[serde(with = "deg_to_rad")]
    pub mean_longitude: f64,
    #[serde(with = "deg_to_rad")]
    pub long_peri: f64,
    #[serde(with = "deg_to_rad")]
    pub long_asc_node: f64,
    pub semi_major_axis_dot: f64,
    pub eccentricity_dot: f64,
    #[serde(with = "deg_to_rad")]
    pub inclination_dot: f64,
    #[serde(with = "deg_to_rad")]
    pub mean_longitude_dot: f64,
    #[serde(with = "deg_to_rad")]
    pub long_peri_dot: f64,
    #[serde(with = "deg_to_rad")]
    pub long_asc_node_dot: f64,
    pub reference_time: DateTime<Utc>,
}

mod deg_to_rad {
    use crate::utils::modulo_2pi;
    use serde::{self, Deserialize, Deserializer};

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let deg = f64::deserialize(deserializer)?;
        let rad = modulo_2pi(2.0 * std::f64::consts::PI * deg / 360.0);
        Ok(rad)
    }
}

impl OrbitParameters {
    pub fn update_parameters_at(self, date: DateTime<Utc>) -> OrbitParameters {
        let duration: Duration = date.signed_duration_since(self.reference_time);
        let centuries: f64 =
            (duration.num_days() as f64 + duration.num_hours() as f64 / 24.0) / 36525.0;
        return OrbitParameters {
            semi_major_axis: self.semi_major_axis + self.semi_major_axis_dot * centuries,
            eccentricity: self.eccentricity + self.eccentricity_dot * centuries,
            inclination: modulo_2pi(self.inclination + self.inclination_dot * centuries),
            mean_longitude: modulo_2pi(self.mean_longitude + self.mean_longitude_dot * centuries),
            long_peri: modulo_2pi(self.long_peri + self.long_peri_dot * centuries),
            long_asc_node: modulo_2pi(self.long_asc_node + self.long_asc_node_dot * centuries),
            reference_time: date,
            ..self.clone()
        };
    }
}

#[derive(Clone)]
pub struct OrbitCourse {
    pub object_name: String,
    pub true_anomaly: f64,
    pub mean_anomaly: f64,
}

pub fn load_orbit_parameters_database(
    file_path: &str,
) -> Result<Vec<OrbitParameters>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut orbits: Vec<OrbitParameters> = vec![];
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let orbit = result?;
        orbits.push(orbit);
    }
    Ok(orbits)
}
