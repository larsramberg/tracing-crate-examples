use std::{thread, time::Duration};

use tracing::{Span, instrument};
use tracing_opentelemetry::OpenTelemetrySpanExt;

#[derive(Debug)]
pub struct Yak {
    wool_quantity: i8,
    is_mean: bool,
}

impl Yak {
    pub fn new(wool_quantity: i8, is_mean: bool) -> Self {
        Self {
            wool_quantity,
            is_mean,
        }
    }

    #[instrument]
    pub(crate) fn shave(&mut self) -> bool {
        thread::sleep(Duration::from_secs(1));

        if self.is_mean {
            tracing::warn!("I will not be shaved, peasant!");
            Span::current().set_status(opentelemetry::trace::Status::Error { description: "Yak was too mean to shave".into() });
            return false;
        }

        tracing::info!("I was shaved!");
        Span::current().set_status(opentelemetry::trace::Status::Ok);
        self.wool_quantity = 0;
        true
    }
}
