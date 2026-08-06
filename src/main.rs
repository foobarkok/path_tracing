use std::io::{self, Write};

mod color;
mod vec3;
use color::write_color;
use vec3::Vec3;

fn main() {
    let width: u32 = 256;
    let height: u32 = 256;

    println!("P3");
    println!("{width}");
    println!("{height}");
    println!("255");

    for j in 0..height {
        eprint!("\r\x1B[2KScanlines remaining:{}", height - j);
        io::stderr().flush().unwrap();
        for i in 0..width {
            let color = Vec3::new(
                i as f64 / (width - 1) as f64,
                j as f64 / (height - 1) as f64,
                0.0,
            );

            write_color(color);
        }
    }
    eprintln!();
    eprintln!("Done.");
}
