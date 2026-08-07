use crate::ray::Ray;
use crate::vec3::Vec3;

struct hit_record {
    pub p: Vec3,
    pub normal: Vec3,
    t: f64,
}

pub trait hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<hit_record>;
}
