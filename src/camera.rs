use crate::{color::write_color, hittable::Hittable, interval::Interval, ray::Ray, vec3::Vec3};
use std::{
    f64,
    io::{self, Write},
};

pub struct Camera {
    image_width: u32,
    image_height: u32,

    samples_per_pixel: u32,
    max_depth: u32,

    defocus_angle: f64,
    focus_dist: f64,

    center: Vec3,        // Camera center
    pixel00_loc: Vec3,   // Location of pixel 0, 0
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel below
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        vfov: f64,
        lookform: Vec3,
        lookat: Vec3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
        let image_height: u32 = if image_height < 1 { 1 } else { image_height };

        let center = lookform;

        // Viewport
        let viewport_height: f64 =
            2.0 * focus_dist * ((vfov * (f64::consts::PI / 180.0)) / 2.0).tan();
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (lookform - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center - w * focus_dist - viewport_u * 0.5 - viewport_v * 0.5;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_dist * ((defocus_angle / 2.0) * (f64::consts::PI / 180.0)).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            image_width,
            image_height,
            samples_per_pixel: 100,
            max_depth: 50,
            defocus_angle,
            focus_dist,
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
            if let Some(scattered) = rec.mat.scatter(r, &rec) {
                return self.ray_color(&scattered.scattered_ray, depth - 1, world)
                    * scattered.attenuation;
            }
            return Vec3::zero();
        }

        let a = (r.direction().unit().y + 1.0) * 0.5;
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + self.pixel_delta_u * (i as f64 + offset.x)
            + self.pixel_delta_v * (j as f64 + offset.y);
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        Ray::new(ray_origin, pixel_sample - ray_origin)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        self.center + self.defocus_disk_u * p.x + self.defocus_disk_v * p.y
    }
}

fn sample_square() -> Vec3 {
    Vec3::new(
        rand::random::<f64>() - 0.5,
        rand::random::<f64>() - 0.5,
        0.0,
    )
}
