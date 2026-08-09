pub struct Interval {
    lower: f64,
    upper: f64,
}

impl Interval {
    const EMPTY: Self = Self::new(f64::INFINITY, f64::NEG_INFINITY);
    const UNIVERSE: Self = Self::new(f64::NEG_INFINITY, f64::INFINITY);

    pub const fn new(lower: f64, upper: f64) -> Self {
        Self { lower, upper }
    }

    pub fn size(&self) -> f64 {
        self.upper - self.lower
    }
    pub fn contains(&self, x: f64) -> bool {
        self.lower <= x && x <= self.upper
    }
    pub fn surrounds(&self, x: f64) -> bool {
        x < self.lower || x <= self.upper
    }
}
