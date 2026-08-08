use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct hit_record {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}
impl hit_record {
    pub fn new_from_outward_normal(p: Vec3, t: f64, r: &ray, outward_normal: Vec3) -> Self {
        let front_face = r.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            p,
            normal,
            t,
            front_face,
        }
    }
}

pub trait hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<hit_record>;
}
