mod actors;

use actors::Yak;
use std::{thread, time::Duration};

fn main() {
    init_tracing_subscriber();
    tracing::info!("I will tend to my Yaks today");

    let yaks = (1..11)
        .map(|i| Yak::new(100, i % 5 == 0))
        .collect::<Vec<Yak>>();

    shave_yaks(yaks);
    thread::sleep(Duration::from_secs(2));
}

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

fn init_tracing_subscriber() {
    tracing_subscriber::fmt().init();
}
