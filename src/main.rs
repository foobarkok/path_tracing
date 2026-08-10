mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;
mod world;
use camera::Camera;
use std::sync::Arc;
use vec3::Vec3;
use world::World;

use crate::{
    material::{Dielectric, Lambertian, Material, Metal},
    sphere::Sphere,
};

fn main() {
    // World
    let mut world = World::new();

    // ground
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = fastrand::f64();
            let center = Vec3::new(
                a as f64 + 0.9 * fastrand::f64(),
                0.2,
                b as f64 + 0.9 * fastrand::f64(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let mat: Arc<dyn Material> = if choose_mat < 0.8 {
                    let albedo = Vec3::random(0.0, 1.0) * Vec3::random(0.0, 1.0);
                    Arc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random(0.5, 1.0);
                    let fuzz = fastrand::f64();
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    Arc::new(Dielectric::new(1.5))
                };
                world.add(Box::new(Sphere::new(center, 0.2, mat)));
            }
        }
    }

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));

    let cam = Camera::new(
        16.0 / 9.0,
        1200,
        20.0,
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::zero(),
        Vec3::new(0.0, 1.0, 0.0),
        0.6,
        10.0,
    )
    .set_samples_per_pixel(500)
    .set_max_depth(50);

    cam.render(&world);
}
