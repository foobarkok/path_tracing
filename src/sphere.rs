use crate::hittable::{hit_record, hittable};
use crate::vec3::Vec3;

struct sphere {
    center: Vec3,
    radius: f64,
}

impl sphere {
    fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}
impl sphere for hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<hit_record> {
        let oc = center - r.origin();
        let a = r.direction().length_squared();
        let h = r.direction().dot(oc);
        let c = oc.length_squared() - radius * radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return None;
            }
        }

        let p = r.at(t);
        let rec = hit_record {
            p,
            t: root,
            normal: (p - center) / radius,
        };

        Some(rec)
    }
}
