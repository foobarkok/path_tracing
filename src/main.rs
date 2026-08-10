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
use std::rc::Rc;
use vec3::Vec3;
use world::World;

use crate::{
    material::{Dielectric, Lambertian, Material, Metal},
    sphere::Sphere,
};

fn main() {
    // World
    let mut world = World::new();

    let material_ground: Rc<dyn Material> = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center: Rc<dyn Material> = Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left: Rc<dyn Material> = Rc::new(Dielectric::new(1.50));
    let material_right: Rc<dyn Material> = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::clone(&material_ground),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.2),
        0.5,
        Rc::clone(&material_center),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&material_left),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&material_right),
    )));

    let cam = Camera::new_from_width_and_ratio(400, 16.0 / 9.0)
        .set_samples_per_pixel(100)
        .set_max_depth(50);

    cam.render(&world);
}
