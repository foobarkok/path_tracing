use glam::Vec3A;

#[derive(Debug, Clone, PartialEq)]
pub struct Ray {
    orig: Vec3A,
    dir: Vec3A,
}

impl Ray {
    pub const fn new(origin: Vec3A, direction: Vec3A) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    pub const fn origin(&self) -> Vec3A {
        self.orig
    }
    pub const fn direction(&self) -> Vec3A {
        self.dir
    }

    pub fn at(&self, t: f32) -> Vec3A {
        self.orig + self.dir * t
    }
}
