use crate::bikes::*;

#[derive(Default)]
struct Parent {
    name: String,
    children: Vec<Children>
}

impl Parent {
    fn set_empty_defaults(&mut self, name: String) {
        self.name = name;
        self.children = Vec::new();
    }
}

struct Children {
    name: String,
    children: Vec<Child>
}

struct Child {
    name: String,
    size: i16
}

fn get_json_for_tree(_bike_list: Bikes) -> Parent {
    let mut parent = Parent {..Default::default()};
    parent.set_empty_defaults(String::from("rusty_bikes"));

    // if bike_list.bikes.len() > 0 {
    //     for item in bike_list.bikes {
            
    //         let mut child = Child {
    //             name: item.bike.manufacturer,
    //             size: 1234
    //         };

    //         parent.children.push(child)
    //     }
    // }

    parent
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_json_for_tree() {

        // bulid test data structure
        let problem = Problem {
            component: String::from("component"),
            description: String::from("description"),
            resolution: String::from("resolution")
        };

        let bike_data = BikeData {
            manufacturer: String::from("manufacturer"),
            model: String::from("model"),
            year: 2022,
            month: 01,
            problems: vec![problem]
        };

        let bike = Bike {
            bike: bike_data
        };

        let bikes_list = Bikes {
            bikes: vec![bike]
        };

        let parent = get_json_for_tree(bikes_list);
        assert!(parent.name == "rusty_bikes");
    }
}