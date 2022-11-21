use opentelemetry::sdk::Resource;
use opentelemetry::{sdk::trace as sdktrace, trace::TraceError};

use opentelemetry_otlp::WithExportConfig;
use opentelemetry_semantic_conventions as semcov;

pub fn init_tracer() -> Result<sdktrace::Tracer, TraceError> {
    let resource = Resource::new(vec![semcov::resource::SERVICE_NAME.string("rusty_bike")]);

    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic().with_env())
        .with_trace_config(sdktrace::config().with_resource(resource))
        .install_simple()
}
