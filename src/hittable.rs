use crate::vec3::Vec3;
use ray::Ray;

struct hit_record {
    pub p: Vec3,
    pub normal: Vec3,
    t: f64,
}

pub trait hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &hit_record) -> bool;
}
