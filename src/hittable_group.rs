use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

pub struct HittableGroup<T: Hittable> {
    objects: Vec<T>,
}

impl<T: Hittable> HittableGroup<T> {
    #[allow(dead_code)]
    pub const fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn new_from_vec(objects: Vec<T>) -> Self {
        Self { objects }
    }

    pub fn add(&mut self, object: T) {
        self.objects.push(object);
    }
}
impl<T: Hittable> Hittable for HittableGroup<T> {
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
