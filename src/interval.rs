use crate::INFINITY;

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: -INFINITY,
            max: INFINITY,
        }
    }
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, t: f64) -> bool {
        t >= self.min && t <= self.max
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}

const INTERVAL_EMPTY: Interval = Interval {
    min: INFINITY,
    max: -INFINITY,
};
const INTERVAL_UNIVERSE: Interval = Interval {
    min: -INFINITY,
    max: INFINITY,
};
