use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

pub struct World {
    objects: Vec<Box<dyn Hittable>>,
}

impl World {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn new_from_vec(objects: Vec<Box<dyn Hittable>>) -> Self {
        Self { objects }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}
impl Hittable for World {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut result: Option<HitRecord> = None;
        let mut closest = ray_t.max;

        for obj in &self.objects {
            if let Some(rec) = obj.hit(r, Interval::new(ray_t.min, closest)) {
                closest = rec.t;
                result = Some(rec);
            }
        }

        result
    }
}
