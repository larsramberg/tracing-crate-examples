mod actors;

use actors::Yak;
use opentelemetry::{KeyValue, trace::TracerProvider as _};
use opentelemetry_otlp::{Protocol, WithExportConfig};
use opentelemetry_sdk::Resource;
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;
use std::{thread, time::Duration};
use tracing::instrument;
use tracing_subscriber::{Registry, layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    init_tracing();
    tracing::info!("I will tend to my Yaks today");

    let yaks = (1..10)
        .map(|i| Yak::new(100, i % 4 == 0))
        .collect::<Vec<Yak>>();
    shave_yaks(yaks);

    thread::sleep(Duration::from_secs(20));
}

#[instrument]
fn shave_yaks(mut yaks: Vec<Yak>) {
    for yak in &mut yaks {
        tracing::info!("I am shaving the yak...");
        if yak.shave() {
            tracing::info!("I managed to collect yak wool!");
        } else {
            tracing::warn!("Oh no, the yak is too mean to shave!");
        }
    }
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
        .with_resource(
            Resource::builder()
                .with_attribute(KeyValue::new(SERVICE_NAME, "yak_shaver"))
                .build(),
        )
        .build();

    let tracer = tracer_provider.tracer("tracer_0");

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let subscriber = Registry::default().with(telemetry);

    subscriber.init();
}
