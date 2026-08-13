use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use glam::Vec3A;
use std::sync::Arc;

pub struct HitRecord {
    pub p: Vec3A,
    pub normal: Vec3A,
    pub mat: Arc<dyn Material>,
    pub t: f32,
    pub front_face: bool,
}
impl HitRecord {
    pub fn new_from_outward_normal(
        p: Vec3A,
        t: f32,
        r: &Ray,
        outward_normal: Vec3A,
        mat: Arc<dyn Material>,
    ) -> Self {
        let front_face = r.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            p,
            normal,
            mat,
            t,
            front_face,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
