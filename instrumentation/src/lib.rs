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
    pub fn shave(&mut self) {
        if self.is_mean {
            warn!("The yak fights back, refusing to be groomed");
            return;
        }
        info!("The yak was sucesfully shaved!");

        self.wool_quantity = 0;
    }
}
