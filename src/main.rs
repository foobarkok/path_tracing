mod camera;
mod color;
mod hittable;
mod hittable_group;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec_util;
mod world;
use camera::Camera;
use glam::Vec3A;
use hittable_group::HittableGroup;
use std::sync::Arc;
use world::World;

use crate::{
    material::{Dielectric, Lambertian, Material, Metal},
    sphere::Sphere,
};

fn render_default() {
    // World
    let mut world = World::new();

    // ground
    world.add(Box::new(Sphere::new(
        Vec3A::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Vec3A::new(0.5, 0.5, 0.5))),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = fastrand::f32();
            let center = Vec3A::new(
                a as f32 + 0.9 * fastrand::f32(),
                0.2,
                b as f32 + 0.9 * fastrand::f32(),
            );
            if (center - Vec3A::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let mat: Arc<dyn Material> = if choose_mat < 0.8 {
                    let albedo = vec_util::random(0.0, 1.0) * vec_util::random(0.0, 1.0);
                    Arc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    let albedo = vec_util::random(0.5, 1.0);
                    let fuzz = fastrand::f32();
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    Arc::new(Dielectric::new(1.5))
                };
                world.add(Box::new(Sphere::new(center, 0.2, mat)));
            }
        }
    }

    world.add(Box::new(Sphere::new(
        Vec3A::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Box::new(Sphere::new(
        Vec3A::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Vec3A::new(0.4, 0.2, 0.1))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3A::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Vec3A::new(0.7, 0.6, 0.5), 0.0)),
    )));

    let cam = Camera::new(
        16.0 / 9.0,
        1200,
        20.0,
        Vec3A::new(13.0, 2.0, 3.0),
        Vec3A::ZERO,
        Vec3A::new(0.0, 1.0, 0.0),
        0.6,
        10.0,
    )
    .set_samples_per_pixel(500)
    .set_max_depth(50);

    cam.render(&world);
}

fn render_with_group() {
    // World
    let mut world = World::new();

    let mut sphere_group = HittableGroup::<Sphere>::new();

    // ground
    sphere_group.add(Sphere::new(
        Vec3A::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Vec3A::new(0.5, 0.5, 0.5))),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = fastrand::f32();
            let center = Vec3A::new(
                a as f32 + 0.9 * fastrand::f32(),
                0.2,
                b as f32 + 0.9 * fastrand::f32(),
            );
            if (center - Vec3A::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let mat: Arc<dyn Material> = if choose_mat < 0.8 {
                    let albedo = vec_util::random(0.0, 1.0) * vec_util::random(0.0, 1.0);
                    Arc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    let albedo = vec_util::random(0.5, 1.0);
                    let fuzz = fastrand::f32();
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    Arc::new(Dielectric::new(1.5))
                };
                sphere_group.add(Sphere::new(center, 0.2, mat));
            }
        }
    }

    sphere_group.add(Sphere::new(
        Vec3A::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    sphere_group.add(Sphere::new(
        Vec3A::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Vec3A::new(0.4, 0.2, 0.1))),
    ));
    sphere_group.add(Sphere::new(
        Vec3A::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Vec3A::new(0.7, 0.6, 0.5), 0.0)),
    ));

    world.add(Box::new(sphere_group));

    let cam = Camera::new(
        16.0 / 9.0,
        1200,
        20.0,
        Vec3A::new(13.0, 2.0, 3.0),
        Vec3A::ZERO,
        Vec3A::new(0.0, 1.0, 0.0),
        0.6,
        10.0,
    )
    .set_samples_per_pixel(500)
    .set_max_depth(50);

    cam.render(&world);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "default" {
        eprintln!("default mode");
        render_default();
    } else {
        eprintln!("group mode");
        render_with_group();
    }
}
