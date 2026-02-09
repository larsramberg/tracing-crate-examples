mod actors;
use std::{thread, time::Duration};

use actors::Yak;

fn main() {
    println!("I will tend to my Yaks today");

    let yaks = (1..10)
        .map(|i| Yak::new(100, i % 4 == 0))
        .collect::<Vec<Yak>>();

    shave_yaks(yaks);

    thread::sleep(Duration::from_secs(20));
}

fn shave_yaks(mut yaks: Vec<Yak>) {
    for yak in &mut yaks {
        println!("I am shaving the yak...");
        if yak.shave() {
            println!("I managed to collect yak wool!");
        } else {
            println!("Oh no, the yak is too mean to shave!");
        }
    }
}