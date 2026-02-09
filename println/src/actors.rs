use std::{thread, time::Duration};

#[derive(Debug)]
pub(crate) struct Yak {
    wool_quantity: i8,
    is_mean: bool,
}

impl Yak {
    pub(crate) fn new(wool_quantity: i8, is_mean: bool) -> Self {
        Self {
            wool_quantity,
            is_mean,
        }
    }

    pub(crate) fn shave(&mut self) -> bool {
        thread::sleep(Duration::from_secs(1));

        if self.is_mean {
            println!("I will not be shaved, peasant!");
            return false;
        }

        println!("I was shaved!");
        self.wool_quantity = 0;
        true
    }
}
