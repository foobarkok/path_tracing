use std::io::{self, Write};

mod color;
mod hittable;
mod interval;
mod ray;
mod sphere;
mod vec3;
mod world;
use color::write_color;
use hittable::Hittable;
use ray::Ray;
use vec3::Vec3;
use world::World;

fn ray_color<T: Hittable>(r: &Ray, world: &T) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.0, 1000.0) {
        return (rec.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let a = (r.direction().unit().y + 1.0) * 0.5;
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let image_height: u32 = if image_height < 1 { 1 } else { image_height };

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

    // Viewport
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Vec3::zero();

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u * 0.5 - viewport_v * 0.5;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    println!("P3");
    println!("{image_width}");
    println!("{image_height}");
    println!("255");

    for j in 0..image_height {
        eprint!("\r\x1B[2KScanlines remaining:{}", image_height - j);
        io::stderr().flush().unwrap();
        for i in 0..image_width {
            let pixel_center = pixel00_loc + pixel_delta_u * i as f64 + pixel_delta_v * j as f64;
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&r, &world);

            write_color(pixel_color);
        }
    }
    eprintln!();
    eprintln!("Done.");
}
