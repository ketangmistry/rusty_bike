use std::{env, fs::File, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Bikes {
    pub bikes: Vec<Bike>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bike {
    pub bike: BikeData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BikeData {
    pub manufacturer: String,
    pub model: String,
    pub year: i16,
    pub month: i8,
    pub problems: Vec<Problem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Problem {
    pub component: String,
    pub description: String,
    pub resolved: bool,
}

pub fn get_bikes() -> Bikes {
    let yaml_file_path = match env::var("BIKES_YAML_FILE") {
        Ok(value) => PathBuf::from(value),
        Err(_) => PathBuf::from("./src/feeds/bikes.yaml"),
    };

    log::info!("bikes yaml file set to {:?}", yaml_file_path.as_os_str());

    let yaml_file = File::open(yaml_file_path).expect("the file bikes.yaml could not be found!");
    let bike_list: Bikes = serde_yaml::from_reader(yaml_file).expect("Could not read values.");
    bike_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bikes() {
        env::set_var("BIKES_YAML_FILE", "./src/feeds/bikes.yaml");
        let bike_list = get_bikes();
        assert!(bike_list.bikes.len() > 0);
        assert!(bike_list.bikes[0].bike.month > 0);
    }
}
