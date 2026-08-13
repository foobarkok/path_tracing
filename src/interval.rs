#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    #[allow(dead_code)]
    const EMPTY: Self = Self::new(f32::INFINITY, f32::NEG_INFINITY);
    #[allow(dead_code)]
    const UNIVERSE: Self = Self::new(f32::NEG_INFINITY, f32::INFINITY);

    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    #[allow(dead_code)]
    pub fn size(&self) -> f32 {
        self.max - self.min
    }
    #[allow(dead_code)]
    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }
    #[allow(dead_code)]
    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }
    #[allow(dead_code)]
    pub fn clamp(&self, x: f32) -> f32 {
        x.clamp(self.min, self.max)
    }
}
