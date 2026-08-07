use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct hit_record {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<hit_record>;
}
