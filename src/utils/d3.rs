use crate::bikes::Bikes;

struct Parent {
    name: String,
    children: Vec<Children>
}

struct Children {
    name: String,
    children: Vec<Child>
}

struct Child {
    name: String,
    size: i16
}

fn get_json_for_tree(bike_list: Bikes) {
    let mut out = String::new();
    out.push_str("There are ");
    out.push_str(&bike_list.bikes.len().to_string());
    out.push_str(" bikes in the data file.");
    println!("{}",out);
}