#[macro_use]
extern crate rocket;

use opentelemetry::Key;
use rocket::fs::{relative, FileServer};
use rocket::response::content::RawJson;

use opentelemetry::sdk::Resource;
use opentelemetry::{
    global, sdk::trace as sdktrace, trace::TraceContextExt, trace::TraceError, trace::Tracer,
};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_semantic_conventions as semcov;

mod data;
use data::bikes;

mod utils;
use utils::d3;

fn init_tracer() -> Result<sdktrace::Tracer, TraceError> {
    let resource = Resource::new(vec![semcov::resource::SERVICE_NAME.string("rusty_bike")]);

    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic().with_env())
        .with_trace_config(sdktrace::config().with_resource(resource))
        .install_simple()
}

#[get("/data")]
fn get_data() -> RawJson<String> {
    let tracer = global::tracer("rusty_bike_data_service");

    let mut d3_json_string = String::new();
    tracer.in_span("get_data", |cx| {
        let span = cx.span();

        span.add_event(
            "event1",
            vec![Key::new("phase1").string("get bikes from yaml file")],
        );
        let bikes = bikes::get_bikes();
        span.set_attribute(Key::new("bike_count").i64(bikes.bikes.len().try_into().unwrap()));

        span.add_event(
            "event2",
            vec![Key::new("phase2").string("get bike object hierarchy")],
        );
        let d3_object = d3::get_d3_root_from_bikes(&bikes);

        span.add_event(
            "event3",
            vec![Key::new("phase3").string("deserialization bike object")],
        );
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
