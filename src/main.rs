mod orbit;
mod position;
mod predictor;

use chrono::Utc;
use std::error::Error;
use std::vec::Vec;

use crate::orbit::{load_orbit_parameters_database, update_orbit_parameters_database_at};
use crate::position::Position;
use crate::predictor::build_predictor;

fn main() -> Result<(), Box<dyn Error>> {
    let orbits_database = load_orbit_parameters_database("orbits.csv".to_string())?;
    let orbits_updated = update_orbit_parameters_database_at(orbits_database, Utc::now());

    let mut positions: Vec<Position> = vec![];
    for orb in orbits_updated.into_iter() {
        let mut prd = build_predictor(orb);
        positions.push(prd.predict());
        print!(
            "{planet}: {steps} steps, {epsilon}\n",
            planet = prd.get_object_name(),
            steps = prd.get_steps(),
            epsilon = prd.get_epsilon()
        );
    }
    Ok(())
}
