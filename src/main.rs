#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket::response::content::RawJson;

use opentelemetry::{global, trace::TraceContextExt, trace::Tracer, Key};

mod data;
use data::bikes;
use data::d3;

mod internals;
use internals::logging;
use internals::tracing;
use rocket_prometheus::PrometheusMetrics;

#[get("/data")]
fn get_data() -> RawJson<String> {
    let tracer = global::tracer("rusty_bike_data_service");

    let mut d3_json_string = String::new();
    tracer.in_span("get_data", |cx| {
        let span = cx.span();

        log::info!("traceID={}", span.span_context().trace_id());

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
    let _tracer = tracing::init_tracer()?;

    match logging::init_logger() {
        Ok(config) => {
            println!("successfully configured logging");
            log4rs::init_config(config)?;
        }
        Err(error) => println!("could not configiure logging, because of {}", error),
    }

    let prometheus = PrometheusMetrics::new();
    
    let _rocket = rocket::build()
        .attach(prometheus.clone()).mount("/metrics", prometheus)
        .mount("/", routes![get_data])
        .mount("/", FileServer::from(relative!("./src/static")))
        .launch()
        .await?;

    Ok(())
}
