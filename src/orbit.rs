use std::error::Error;
use std::fs::File;
use std::vec::Vec;

use chrono::{DateTime, Duration, Utc};
use csv;
use serde::Deserialize;

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
        let rad = 2.0 * std::f64::consts::PI * deg / 360.0;
        Ok(rad)
    }
}

impl OrbitParameters {
    pub fn update_parameters_at(self, date: DateTime<Utc>) -> OrbitParameters {
        let duration: Duration = date.signed_duration_since(self.reference_time);
        let centuries: f64 = duration.num_hours() as f64 / 24.0 / 36525.0;
        return OrbitParameters {
            semi_major_axis: self.semi_major_axis + self.semi_major_axis_dot * centuries,
            eccentricity: self.eccentricity + self.eccentricity_dot * centuries,
            inclination: self.inclination + self.inclination_dot * centuries,
            mean_longitude: self.mean_longitude + self.mean_longitude_dot * centuries,
            long_peri: self.long_peri + self.long_peri_dot * centuries,
            long_asc_node: self.long_asc_node + self.long_asc_node_dot * centuries,
            reference_time: date,
            ..self.clone()
        };
    }

    pub fn get_orbit_box(&self, mult: f64) -> ([f64; 4], f64) {
        return (
            [
                -self.semi_major_axis * self.eccentricity * mult, // x0
                0.0,                                              // y0
                2.0 * self.semi_major_axis * self.inclination.cos() * mult, // width, w/ projection
                2.0 * self.semi_major_axis * (1.0 - self.eccentricity) * mult, // height
            ],
            -self.long_peri, // rotation around z
        );
    }
}

#[derive(Clone)]
pub struct OrbitCourse {
    pub true_anomaly: f64,
    pub mean_anomaly: f64,
}

pub fn load_orbit_parameters_database(
    file_path: String,
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

pub fn update_orbit_parameters_database_at(
    orbits: Vec<OrbitParameters>,
    date: DateTime<Utc>,
) -> Vec<OrbitParameters> {
    let mut orbits_updated: Vec<OrbitParameters> = vec![];
    for o in orbits.into_iter() {
        orbits_updated.push(o.update_parameters_at(date));
    }
    return orbits_updated;
}
