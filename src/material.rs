use crate::{hittable::HitRecord, ray::Ray, vec3::Vec3};

pub struct Scattered {
    pub attenuation: Vec3,
    pub scattered_ray: Ray,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattered>;
}

pub struct Lambertian {
    albedo: Vec3,
}
impl Lambertian {
    fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}
impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattered> {
        let dir = rec.normal + Vec3::random_unit_vector();
        Some(Scattered {
            attenuation: self.albedo,
            scattered_ray: Ray::new(rec.p, dir),
        })
    }
}
