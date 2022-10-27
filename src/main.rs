#[macro_use]
extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket::response::content::RawHtml;

mod data;

use data::bikes;

#[get["/"]]
fn index() -> RawHtml<String> {
    let bike_list = bikes::get_bikes();
    let mut response = String::new();
    response.push_str("There are ");
    response.push_str(&bike_list.bikes.len().to_string());
    response.push_str(" bikes in the data file.");
    RawHtml(response)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/favicon.ico", FileServer::from(relative!("./src/static")))
}
