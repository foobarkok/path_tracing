use crate::{color::write_color, hittable::Hittable, interval::Interval, ray::Ray, vec3::Vec3};
use std::io::{self, Write};

pub struct Camera {
    image_width: u32,
    image_height: u32,

    samples_per_pixel: u32,
    max_depth: u32,

    center: Vec3,        // Camera center
    pixel00_loc: Vec3,   // Location of pixel 0, 0
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel below
}

impl Camera {
    pub fn new_from_width_and_ratio(image_width: u32, aspect_ratio: f64) -> Self {
        let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
        let image_height: u32 = if image_height < 1 { 1 } else { image_height };

        // Viewport
        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
        let center = Vec3::zero();

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u * 0.5 - viewport_v * 0.5;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Self {
            image_width,
            image_height,
            samples_per_pixel: 100,
            max_depth: 50,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render<T: Hittable>(&self, world: &T) {
        let image_width = self.image_width;
        let image_height = self.image_height;

        println!("P3");
        println!("{image_width}");
        println!("{image_height}");
        println!("255");

        for j in 0..image_height {
            eprint!("\r\x1B[2KScanlines remaining:{}", image_height - j);
            io::stderr().flush().unwrap();
            for i in 0..image_width {
                if self.samples_per_pixel == 1 {
                    let pixel_center = self.pixel00_loc
                        + self.pixel_delta_u * i as f64
                        + self.pixel_delta_v * j as f64;
                    let ray_direction = pixel_center - self.center;
                    let r = Ray::new(self.center, ray_direction);
                    let pixel_color = self.ray_color(&r, self.max_depth, world);

                    write_color(pixel_color);
                } else {
                    let pixel_color = (0..self.samples_per_pixel)
                        .map(|_| {
                            let r = self.get_ray(i, j);
                            self.ray_color(&r, self.max_depth, world)
                        })
                        .sum::<Vec3>()
                        / self.samples_per_pixel as f64;

                    write_color(pixel_color);
                }
            }
        }
        eprintln!();
        eprintln!("Done.");
    }

    fn ray_color<T: Hittable>(&self, r: &Ray, depth: u32, world: &T) -> Vec3 {
        if depth <= 0 {
            return Vec3::zero();
        }

        if let Some(rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            let dir = Vec3::random_on_hemisphere(rec.normal);
            return self.ray_color(&Ray::new(rec.p, dir), depth - 1, world) * 0.5;
        }

        let a = (r.direction().unit().y + 1.0) * 0.5;
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + self.pixel_delta_u * (i as f64 + offset.x)
            + self.pixel_delta_v * (j as f64 + offset.y);
        Ray::new(self.center, pixel_sample - self.center)
    }
}

fn sample_square() -> Vec3 {
    Vec3::new(
        rand::random::<f64>() - 0.5,
        rand::random::<f64>() - 0.5,
        0.0,
    )
}
