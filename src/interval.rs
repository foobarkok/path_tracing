#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    #[allow(dead_code)]
    const EMPTY: Self = Self::new(f64::INFINITY, f64::NEG_INFINITY);
    #[allow(dead_code)]
    const UNIVERSE: Self = Self::new(f64::NEG_INFINITY, f64::INFINITY);

    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    #[allow(dead_code)]
    pub fn size(&self) -> f64 {
        self.max - self.min
    }
    #[allow(dead_code)]
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    #[allow(dead_code)]
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
    #[allow(dead_code)]
    pub fn clamp(&self, x: f64) -> f64 {
        x.clamp(self.min, self.max)
    }
}
