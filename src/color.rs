use crate::vec3::Vec3;

pub fn write_color(color: Vec3) {
    let r = (256.0 * color.x.clamp(0.0, 0.999)) as i32;
    let g = (256.0 * color.y.clamp(0.0, 0.999)) as i32;
    let b = (256.0 * color.z.clamp(0.0, 0.999)) as i32;

    println!("{r} {g} {b}");
}
