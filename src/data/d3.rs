use opentelemetry::{global, trace::TraceContextExt, trace::Tracer, Key};

use serde::Serialize;

use crate::bikes::*;

#[derive(Default, Serialize)]
pub struct Root {
    name: String,
    children: Vec<Parent>,
}

impl Root {
    fn get_manufacturer_by_name(&mut self, name: &str) -> Option<&mut Parent> {
        self.children.iter_mut().find(|p| p.name == name)
    }
}

#[derive(Default, Serialize)]
pub struct Parent {
    name: String,
    children: Vec<Child>,
}

impl Parent {
    fn get_model_by_name(&mut self, name: &str) -> Option<&mut Child> {
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
    resolved: bool,
}

impl ChildData {
    fn new(name: String, size: i16, resolved: bool) -> Self {
        ChildData {
            name,
            size,
            resolved,
        }
    }
}

/// From a list of bikes, get an object which can be deserialized into JSON which supports the
/// D3 Collapsible Tree (https://observablehq.com/@d3/collapsible-tree).
pub fn get_d3_root_from_bikes(bikes: &Bikes) -> Root {
    let tracer = global::tracer("rusty_bike_data_service");

    // the root name will be fixed
    let mut root = Root {
        ..Default::default()
    };
    root.name = String::from("rusty bike(s)");

    tracer.in_span("get_d3_root_from_bikes", |cx| {
        let span = cx.span();

        if !bikes.bikes.is_empty() {
            for bike in &bikes.bikes {
                if !bike.bike.problems.is_empty() {
                    for problem in &bike.bike.problems {
                        // set the parent name as component
                        match root.get_manufacturer_by_name(&bike.bike.manufacturer) {
                            Some(parent) => {
                                match parent.get_model_by_name(&bike.bike.model) {
                                    Some(child) => {
                                        // add to existing model
                                        let child_data = ChildData::new(
                                            problem.description.clone(),
                                            1234,
                                            problem.resolved,
                                        );
                                        child.children.push(child_data);
                                    }
                                    None => {
                                        // create new model
                                        let mut child = Child {
                                            ..Default::default()
                                        };
                                        child.name = bike.bike.model.clone();

                                        let event1_text =
                                            String::from("adding new model") + &child.name;
                                        span.add_event(
                                            "event1",
                                            vec![Key::new("phase1").string(event1_text)],
                                        );

                                        let child_data = ChildData::new(
                                            problem.description.clone(),
                                            1234,
                                            problem.resolved,
                                        );
                                        child.children.push(child_data);

                                        parent.children.push(child);
                                    }
                                }
                            }
                            None => {
                                let mut parent = Parent {
                                    ..Default::default()
                                };
                                parent.name = bike.bike.manufacturer.clone();

                                // add a child to parent containing the manufacturer
                                let mut child = Child {
                                    ..Default::default()
                                };
                                child.name = bike.bike.model.clone();

                                // now add a child to child, or grandchild with description
                                let child_data = ChildData::new(
                                    problem.description.clone(),
                                    1234,
                                    problem.resolved,
                                );

                                // set the relationships
                                child.children.push(child_data);
                                parent.children.push(child);

                                root.children.push(parent);
                            }
                        }
                    }
                }
            }
        }
    });

    root
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> Bikes {
        let problem1 = Problem {
            component: String::from("problem1_component"),
            description: String::from("problem1_description"),
            resolved: true,
        };

        let problem2 = Problem {
            component: String::from("problem2_component"),
            description: String::from("problem2_description"),
            resolved: false,
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
        assert!(root.name == "rusty bike(s)");
    }

    #[test]
    fn test_get_problem_component_by_name() {
        let bikes = get_test_data();
        let mut root = get_d3_root_from_bikes(&bikes);
        match root.get_manufacturer_by_name("manufacturer") {
            Some(_parent) => assert!(true),
            None => assert!(false),
        }
    }

    #[test]
    fn test_get_problem_and_manufacturer_by_name() {
        let bikes = get_test_data();
        let mut root = get_d3_root_from_bikes(&bikes);
        match root.get_manufacturer_by_name("manufacturer") {
            Some(parent) => match parent.get_model_by_name("model") {
                Some(_child) => assert!(true),
                None => assert!(false),
            },
            None => assert!(false),
        }
    }
}
