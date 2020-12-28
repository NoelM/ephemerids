use crate::position::Position;
use std::collections::HashMap;
use serde::de::value::StrDeserializer;

pub fn translate_to_earth(positions: HashMap<String, Position>) -> HashMap<String, Position> {
    let earth_position = positions.get("Earth").unwrap();

    return positions.iter()
        .filter( |&(k,_)| k != "Earth")
        .map( |(k, p)| (k.clone(), (p - earth_position).clone()))
        .collect();
}