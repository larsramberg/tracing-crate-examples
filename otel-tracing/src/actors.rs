use std::{thread, time::Duration};

use tracing::{instrument, warn};

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
            warn!("I will not be shaved, peasant!");
            return false;
        }

        tracing::info!("I was shaved!");
        self.wool_quantity = 0;
        true
    }
}
