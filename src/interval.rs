#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    const EMPTY: Self = Self::new(f64::INFINITY, f64::NEG_INFINITY);
    const UNIVERSE: Self = Self::new(f64::NEG_INFINITY, f64::INFINITY);

    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
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
        x.clamp(self.min, self.max)
    }
}
