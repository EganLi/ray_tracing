use crate::rtweekend::infinity;

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn defalut() -> Interval {
        Interval {
            min: infinity,
            max: -infinity,
        }
    }

    pub fn new(_min: f64, _max: f64) -> Interval {
        Interval {
            min: _min,
            max: _max,
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if self.max < x {
            return self.max;
        }
        x
    }
}

const empty: Interval = Interval {
    min: infinity,
    max: -infinity,
};
const universe: Interval = Interval {
    min: -infinity,
    max: infinity,
};
