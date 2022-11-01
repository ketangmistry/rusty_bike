#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket::response::content::RawJson;

mod data;
use data::bikes;

mod utils;
use utils::d3;

#[get["/data"]]
fn get_data() -> RawJson<String> {
    let bike_list = bikes::get_bikes();
    let d3_object = d3::get_object_for_d3_tree(&bike_list);
    let d3_json_string = serde_json::to_string_pretty(&d3_object).unwrap();
    RawJson(d3_json_string)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_data])
        .mount("/", FileServer::from(relative!("./src/static")))
}
