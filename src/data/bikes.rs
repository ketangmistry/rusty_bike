use std::{fs::File, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Bikes {
    pub bikes: Vec<Bike>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bike {
    bike: BikeData,
}

#[derive(Debug, Serialize, Deserialize)]
struct BikeData {
    manufacturer: String,
    model: String,
    year: i16,
    month: i8,
    problems: Vec<Problem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Problem {
    component: String,
    description: String,
    resolution: String,
}

pub fn get_bikes() -> Bikes {
    let yaml_file_path = PathBuf::from("./src/data/bikes.yaml");
    let yaml_file = File::open(yaml_file_path).expect("the file bikes.yaml could not be found!");
    let bike_list: Bikes = serde_yaml::from_reader(yaml_file).expect("Could not read values.");
    bike_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bikes() {
        let bike_list = get_bikes();
        assert!(bike_list.bikes.len() > 0);
        assert!(bike_list.bikes[0].bike.month > 0);
    }
}
