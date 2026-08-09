use crate::interval::Interval;
use crate::vec3::Vec3;

pub fn write_color(color: Vec3) {
    const INTENSITY: Interval = Interval::new(0.0, 0.999);
    let r = (256.0 * INTENSITY.clamp(color.x)) as i32;
    let g = (256.0 * INTENSITY.clamp(color.y)) as i32;
    let b = (256.0 * INTENSITY.clamp(color.z)) as i32;

    println!("{r} {g} {b}");
}
