use crate::vec3::Vec3;

pub fn write_color(color: Vec3) {
    let r = (color.x * 255.99) as i32;
    let g = (color.y * 255.99) as i32;
    let b = (color.z * 255.99) as i32;

    println!("{r} {g} {b}");
}
