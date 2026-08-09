use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct World {
    objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn new_from_vec(objects: Vec<Box<dyn Hittable>>) -> Self {
        Self { objects }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}
impl Hittable for World {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        self.objects
            .iter()
            .find_map(|obj| obj.hit(r, ray_tmin, ray_tmax))
    }
}
