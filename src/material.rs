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
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<Scattered> {
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
        let dir = dir.unit() + Vec3::random_unit_vector() * self.fuzz;
        if dir.dot(rec.normal) > 0.0 {
            Some(Scattered {
                attenuation: self.albedo,
                scattered_ray: Ray::new(rec.p, dir),
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refraction_index: f64,
}
impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}
impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattered> {
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_dir = r_in.direction().unit();
        let cos_theta = -unit_dir.dot(rec.normal);
        let cos_theta = if cos_theta < 1.0 { cos_theta } else { 1.0 };
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let direction =
            if ri * sin_theta > 1.0 || reflectance(cos_theta, ri) > rand::random::<f64>() {
                unit_dir.reflect(rec.normal)
            } else {
                unit_dir.refract(rec.normal, ri)
            };
        Some(Scattered {
            attenuation: Vec3::new(1.0, 1.0, 1.0),
            scattered_ray: Ray::new(rec.p, direction),
        })
    }
}
fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    let x = 1.0 - cosine;
    let x2 = x * x;
    let x5 = x2 * x2 * x;
    r0 + (1.0 - r0) * x5
}
