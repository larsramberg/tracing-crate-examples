use std::{thread, time::Duration};

use tracing::{debug, info, instrument, warn};

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
    fn shave(&mut self) -> bool {
        thread::sleep(Duration::from_secs(1));

        if self.is_mean {
            warn!("I will not be shaved, peasant!");
            return false;
        }
        
        tracing::info!("I was shaved!");
        self.wool_quantity = 0;
        true
    }
}

#[derive(Debug)]
pub struct Farmer {

}

impl Farmer {
    #[instrument]
    pub fn shave(&self, yak: &mut Yak) {
        tracing::info!("I am shaving the yak...");
        if yak.shave() {
            tracing::info!("I managed to collect yak wool!");
        } else {
            tracing::warn!("Oh no, the yak is too mean to shave!");
        }
    }
}