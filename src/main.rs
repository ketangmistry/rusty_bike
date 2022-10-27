mod data;

use data::bikes;

fn main() {
    let bike_list = bikes::get_bikes();
    println!(
        "There are {} bikes in the data file.",
        bike_list.bikes.len()
    );
}
