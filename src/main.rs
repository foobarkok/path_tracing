use std::io::{self, Write};

mod camera;
mod color;
mod hittable;
mod interval;
mod ray;
mod sphere;
mod vec3;
mod world;
use camera::Camera;
use color::write_color;
use hittable::Hittable;
use ray::Ray;
use vec3::Vec3;
use world::World;

use crate::interval::Interval;

fn main() {
    // World
    let mut world = World::new();
    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    let cam = Camera::new_from_width_and_ratio(400, 16.0 / 9.0);

    cam.render(&world);
}
