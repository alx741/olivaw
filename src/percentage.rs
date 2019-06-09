// use ufmt::{derive::uDebug, uDebug};

// #[derive(uDebug)]
pub struct Percentage {
    percent: f32,
}

impl Percentage {
    pub fn new(percent: f32) -> Percentage {
        if percent >= 0.0 && percent <= 100.0 {
            Percentage { percent }
        } else {
            panic!("Percentage value outside [0, 100] range");
        }
    }

    pub fn value(&self) -> f32 {
        self.percent
    }
}
