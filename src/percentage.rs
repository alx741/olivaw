#[derive(Copy, Clone)]
pub struct Percentage {
    percent: f32,
}

impl Percentage {
    pub fn new(percent: f32) -> Percentage {
        if percent < 0.0 {
            Percentage { percent: 0.0 }
        }
        else if percent > 100.0 {
            Percentage { percent: 100.0 }
        } else {
            Percentage { percent }
        }
    }

    pub fn value(&self) -> f32 {
        self.percent
    }
}
