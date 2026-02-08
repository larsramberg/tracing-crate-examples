
use instrumentation::Yak;
use tracing::instrument;

#[instrument]
fn main() {
    init_tracing_subscriber();
    tracing::info!("I will tend to my Yaks today");

    let mut yaks = (1..10).map(|i| Yak::new(i, i % 4==0) ).collect::<Vec<Yak>>();
    
    for yak in &mut yaks {
        yak.shave();
    }
}

fn init_tracing_subscriber() {
    tracing_subscriber::fmt().init();
}