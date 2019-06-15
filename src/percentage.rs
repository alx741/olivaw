#[derive(Copy, Clone)]
pub struct Percentage {
    percent: f32,
}

impl Percentage {
    pub fn new(percent: f32) -> Percentage {
        let mut p = Percentage { percent: 0.0 };
        p.set(percent);
        p
    }

    pub fn set(&mut self, percent: f32) {
        if percent < 0.0 {
            self.percent = 0.0;
        }
        else if percent > 100.0 {
            self.percent = 100.0;
        } else {
            self.percent = percent;
        }
    }

    pub fn value(&self) -> f32 {
        self.percent
    }
}
