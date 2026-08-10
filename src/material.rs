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
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}
impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattered> {
        let mut dir = rec.normal + Vec3::random_unit_vector();
        if dir.near_zero() {
            dir = rec.normal;
        }
        Some(Scattered {
            attenuation: self.albedo,
            scattered_ray: Ray::new(rec.p, dir),
        })
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}
impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Self { albedo, fuzz }
    }
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattered> {
        let dir = r_in.direction().reflect(rec.normal);
        let dir_fuzz = dir.unit() + Vec3::random_unit_vector() * self.fuzz;
        let dir_fuzz = if dir_fuzz.near_zero() { dir } else { dir_fuzz };
        Some(Scattered {
            attenuation: self.albedo,
            scattered_ray: Ray::new(rec.p, dir_fuzz),
        })
    }
}
