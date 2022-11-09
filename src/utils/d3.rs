use serde::Serialize;

use crate::bikes::*;

#[derive(Default, Serialize)]
pub struct Root {
    name: String,
    children: Vec<Parent>,
}

impl Root {
    fn get_problem_component_by_name(&mut self, name: &str) -> Option<&mut Parent> {
        self.children.iter_mut().find(|p| p.name == name)
    }
}

#[derive(Default, Serialize)]
pub struct Parent {
    name: String,
    children: Vec<Child>,
}

impl Parent {
    fn get_manufacturer_by_name(&mut self, name: &str) -> Option<&mut Child> {
        self.children.iter_mut().find(|c| c.name == name)
    }
}

#[derive(Default, Serialize)]
struct Child {
    name: String,
    children: Vec<ChildData>,
}

#[derive(Default, Serialize)]
struct ChildData {
    name: String,
    size: i16,
}

pub fn get_d3_root_from_bikes(bikes: &Bikes) -> Root {
    // the root name will be fixed
    let mut root = Root {
        ..Default::default()
    };
    root.name = String::from("rusty_bikes");

    if !bikes.bikes.is_empty() {
        for x in &bikes.bikes {
            if !x.bike.problems.is_empty() {
                for y in &x.bike.problems {
                    
                    let mut parent = Parent {
                        ..Default::default()
                    };

                    // set the parent name as component
                    match root.get_problem_component_by_name(&y.component) {
                        Some(p) => 
                            match p.get_manufacturer_by_name(&x.bike.manufacturer) {
                                Some(c) => {
                                    // add to existing manufacturer
                                    let mut d = ChildData {
                                        ..Default::default()
                                    };
                                    d.name = y.description.clone();
                                    d.size = 1234;
                                    c.children.push(d)
                                }
                                None => {
                                    // create new manufacturer
                                    let mut c = Child {
                                        ..Default::default()
                                    };
                                    c.name = x.bike.manufacturer.clone();

                                    let mut d = ChildData {
                                        ..Default::default()
                                    };
                                    d.name = y.description.clone();
                                    d.size = 1234;
                                    c.children.push(d)

                                }
                            },
                        None => {}
                    }
                    parent.name = y.component.clone();

                    // add a child to parent containing the manufacturer
                    let mut child = Child {
                        ..Default::default()
                    };
                    child.name = x.bike.manufacturer.clone();

                    // now add a child to child, or grandchild with description
                    let mut child_data = ChildData {
                        ..Default::default()
                    };
                    child_data.name = y.description.clone();
                    child_data.size = 1234;

                    // set the relationships
                    child.children.push(child_data);
                    parent.children.push(child);

                    root.children.push(parent);
                }
            }
        }
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> Bikes {
        let problem1 = Problem {
            component: String::from("problem1_component"),
            description: String::from("problem1_description"),
            resolution: String::from("problem1_resolution"),
        };

        let problem2 = Problem {
            component: String::from("problem2_component"),
            description: String::from("problem2_description"),
            resolution: String::from("problem2_resolution"),
        };

        let bike_data = BikeData {
            manufacturer: String::from("manufacturer"),
            model: String::from("model"),
            year: 2022,
            month: 01,
            problems: vec![problem1, problem2],
        };

        let bike = Bike { bike: bike_data };

        let bikes = Bikes { bikes: vec![bike] };

        bikes

    }

    #[test]
    fn test_get_d3_root_from_bikes() {
        let bikes = get_test_data();
        let root = get_d3_root_from_bikes(&bikes);
        assert!(root.name == "rusty_bikes");
    }

    #[test]
    fn test_get_problem_component_by_name() {
        let bikes = get_test_data();
        let mut root = get_d3_root_from_bikes(&bikes);
        match root.get_problem_component_by_name("problem1_component") {
            Some(_parent) => assert!(true),
            None => assert!(false),
        }
    }

    #[test]
    fn test_get_problem_and_manufacturer_by_name() {
        let bikes = get_test_data();
        let mut root = get_d3_root_from_bikes(&bikes);
        match root.get_problem_component_by_name("problem1_component") {
            Some(parent) => 
                match parent.get_manufacturer_by_name("manufacturer") {
                    Some(_child) => assert!(true),
                    None => assert!(false)
                }
            None => assert!(false),
        }
    }

}
