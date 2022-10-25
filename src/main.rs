mod data;
use crate::data::bikes::Bike;

fn main() {
    println!("Rusty bike!");
    let bike = Bike {make: "e".to_string(), model: 'f'.to_string()};
    println!("{}", bike.make);
}
