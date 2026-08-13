use crate::{color::write_color, hittable::Hittable, interval::Interval, ray::Ray, vec_util};
use glam::Vec3A;
use rayon::prelude::*;
use std::{
    f32,
    io::{self, Write},
};

pub struct Camera {
    image_width: u32,
    image_height: u32,

    samples_per_pixel: u32,
    max_depth: u32,

    defocus_angle: f32,

    center: Vec3A,        // Camera center
    pixel00_loc: Vec3A,   // Location of pixel 0, 0
    pixel_delta_u: Vec3A, // Offset to pixel to the right
    pixel_delta_v: Vec3A, // Offset to pixel below
    defocus_disk_u: Vec3A,
    defocus_disk_v: Vec3A,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        image_width: u32,
        vfov: f32,
        lookform: Vec3A,
        lookat: Vec3A,
        vup: Vec3A,
        defocus_angle: f32,
        focus_dist: f32,
    ) -> Self {
        let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;
        let image_height: u32 = if image_height < 1 { 1 } else { image_height };

        let center = lookform;

        // Viewport
        let viewport_height: f32 =
            2.0 * focus_dist * ((vfov * (f32::consts::PI / 180.0)) / 2.0).tan();
        let viewport_width: f32 = viewport_height * (image_width as f32 / image_height as f32);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (lookform - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center - w * focus_dist - viewport_u * 0.5 - viewport_v * 0.5;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_dist * ((defocus_angle / 2.0) * (f32::consts::PI / 180.0)).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            image_width,
            image_height,
            samples_per_pixel: 100,
            max_depth: 50,
            defocus_angle,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
    #[allow(dead_code)]
    pub fn set_samples_per_pixel(mut self, samples_per_pixel: u32) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }
    #[allow(dead_code)]
    pub fn set_max_depth(mut self, max_depth: u32) -> Self {
        self.max_depth = max_depth;
        self
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
            let pixel_colors: Vec<_> = (0..image_width)
                .into_par_iter()
                .map(|i| {
                    if self.samples_per_pixel == 1 {
                        let pixel_center = self.pixel00_loc
                            + self.pixel_delta_u * i as f32
                            + self.pixel_delta_v * j as f32;
                        let ray_direction = pixel_center - self.center;
                        let r = Ray::new(self.center, ray_direction);
                        self.ray_color(&r, self.max_depth, world)
                    } else {
                        (0..self.samples_per_pixel)
                            .map(|_| {
                                let r = self.get_ray(i, j);
                                self.ray_color(&r, self.max_depth, world)
                            })
                            .sum::<Vec3A>()
                            / self.samples_per_pixel as f32
                    }
                })
                .collect();
            for pixel_color in pixel_colors {
                write_color(pixel_color);
            }
        }
        eprintln!();
        eprintln!("Done.");
    }

    fn ray_color<T: Hittable>(&self, r: &Ray, depth: u32, world: &T) -> Vec3A {
        if depth <= 0 {
            return Vec3A::ZERO;
        }

        if let Some(rec) = world.hit(r, Interval::new(0.001, f32::INFINITY)) {
            if let Some(scattered) = rec.mat.scatter(r, &rec) {
                return self.ray_color(&scattered.scattered_ray, depth - 1, world)
                    * scattered.attenuation;
            }
            return Vec3A::ZERO;
        }

        let a = (r.direction().normalize().y + 1.0) * 0.5;
        Vec3A::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3A::new(0.5, 0.7, 1.0) * a
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + self.pixel_delta_u * (i as f32 + offset.x)
            + self.pixel_delta_v * (j as f32 + offset.y);
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        Ray::new(ray_origin, pixel_sample - ray_origin)
    }

    fn defocus_disk_sample(&self) -> Vec3A {
        let p = vec_util::random_in_unit_disk();
        self.center + self.defocus_disk_u * p.x + self.defocus_disk_v * p.y
    }
}

fn sample_square() -> Vec3A {
    Vec3A::new(fastrand::f32() - 0.5, fastrand::f32() - 0.5, 0.0)
}
