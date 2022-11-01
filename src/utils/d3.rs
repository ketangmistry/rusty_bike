use serde::Serialize;

use crate::bikes::*;

#[derive(Default, Serialize)]
pub struct Root {
    name: String,
    children: Vec<Parent>,
}

#[derive(Default, Serialize)]
pub struct Parent {
    name: String,
    children: Vec<Child>,
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

pub fn get_object_for_d3_tree(bike_list: &Bikes) -> Root {
    // the root name will be fixed
    let mut root = Root {
        ..Default::default()
    };
    root.name = String::from("rusty_bikes");

    if !bike_list.bikes.is_empty() {
        for x in &bike_list.bikes {
            if !x.bike.problems.is_empty() {
                for y in &x.bike.problems {
                    let mut parent = Parent {
                        ..Default::default()
                    };

                    // set the parent name as component
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

    #[test]
    fn test_get_json_for_tree() {
        // bulid test data structure
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

        let bikes_list = Bikes { bikes: vec![bike] };

        let parent = get_object_for_d3_tree(&bikes_list);
        assert!(parent.name == "rusty_bikes");
    }
}
