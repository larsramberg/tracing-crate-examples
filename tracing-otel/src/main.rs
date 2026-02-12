mod actors;

use actors::Yak;
use opentelemetry::{KeyValue, trace::TracerProvider as _};
use opentelemetry_otlp::{Protocol, WithExportConfig};
use opentelemetry_sdk::{Resource, metrics::PeriodicReader};
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;
use tracing_opentelemetry::{OpenTelemetryLayer, MetricsLayer};
use std::{thread, time::Duration};
use tracing::{info, instrument};
use tracing_subscriber::{Registry, fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    init_tracing();
    tracing::info!("I will tend to my Yaks today");

    let yaks = (1..11)
        .map(|i| Yak::new(100, i % 5 == 0))
        .collect::<Vec<Yak>>();

    shave_yaks(yaks);
    thread::sleep(Duration::from_secs(10));
}

#[instrument]
fn shave_yaks(mut yaks: Vec<Yak>) {
    let mut yaks_shaven = 0;
    for yak in &mut yaks {
        tracing::info!("I am shaving the yak...");
        if yak.shave() {
            tracing::info!("I managed to collect yak wool!");
            yaks_shaven += 1;
            info!(monotonic_counter.yaks_shaven = yaks_shaven);
        } else {
            tracing::warn!("Oh no, the yak is too mean to shave!");
        }
    }
}

fn init_tracing() {
    let fmt_layer = fmt::layer();
    
    let exporter = opentelemetry_otlp::MetricExporter::builder().with_http().build().unwrap();
    let reader = PeriodicReader::builder(exporter).with_interval(Duration::from_secs(1)).build();
    let provider = opentelemetry_sdk::metrics::SdkMeterProvider::builder().with_reader(reader).build();
    let metrics_layer = MetricsLayer::new(provider);

    let subscriber = 
        Registry::default()
        .with(get_trace_layer())
        .with(metrics_layer)
        .with(fmt_layer);

    subscriber.init();
}

fn get_trace_layer() -> OpenTelemetryLayer<Registry, opentelemetry_sdk::trace::Tracer> {
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

    tracing_opentelemetry::layer().with_tracer(tracer)
}
