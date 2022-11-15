#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket::response::content::RawJson;

use opentelemetry::sdk::Resource;
use opentelemetry::trace::TraceError;
use opentelemetry::{global, sdk::trace as sdktrace};
use opentelemetry::{trace::Tracer};
use opentelemetry_otlp::WithExportConfig;

mod data;
use data::bikes;

mod utils;
use utils::d3;

fn init_tracer() -> Result<sdktrace::Tracer, TraceError> {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic().with_env())
        .with_trace_config(
            sdktrace::config().with_resource(Resource::default()),
        )
        .install_simple()
}

#[get("/data")]
fn get_data() -> RawJson<String> {

    let tracer = global::tracer("rusty_bike_data_service");

    let mut d3_json_string = String::new();
    tracer.in_span("get_bike_data", |_cx| {
        let bikes = bikes::get_bikes();
        let d3_object = d3::get_d3_root_from_bikes(&bikes);
        d3_json_string = serde_json::to_string_pretty(&d3_object).unwrap();
    });

    RawJson(d3_json_string)
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let _tracer = init_tracer()?;

    let _rocket = rocket::build()
        .mount("/", routes![get_data])
        .mount("/", FileServer::from(relative!("./src/static")))
        .launch()
        .await?;

    Ok(())
}
