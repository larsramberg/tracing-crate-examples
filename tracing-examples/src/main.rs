use std::{thread, time::Duration};

use instrumentation::{Farmer, Yak};
use opentelemetry::{
    KeyValue,
    trace::{self, Tracer, TracerProvider as _},
};
use opentelemetry_otlp::{Protocol, WithExportConfig};
use opentelemetry_sdk::Resource;
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;
use tracing::instrument;
use tracing_subscriber::{Registry, layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    init_tracing();
    tracing::info!("I will tend to my Yaks today");

    let mut yaks = (1..10)
        .map(|i| Yak::new(i, i % 4 == 0))
        .collect::<Vec<Yak>>();

    let farmer = Farmer{};

    for yak in &mut yaks {
        farmer.shave(yak)
    }

    thread::sleep(Duration::from_secs(20));
}

fn init_tracing() {
    let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpBinary)
        .build()
        .unwrap();

    // Create a tracer provider with the exporter
    let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_batch_exporter(otlp_exporter)
        .with_resource(Resource::from(
            Resource::builder()
                .with_attribute(KeyValue::new(SERVICE_NAME, "yak_shaver"))
                .build(),
        ))
        .build();

    let tracer = tracer_provider.tracer("tracer_0");

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let subscriber = Registry::default().with(telemetry);

    subscriber.init();
}
