use glam::Vec3A;

fn liner2gamma(liner: f32) -> f32 {
    if liner > 0.0 {
        return liner.sqrt();
    }
    0.0
}

pub fn write_color(color: Vec3A) {
    let r = liner2gamma(color.x);
    let g = liner2gamma(color.y);
    let b = liner2gamma(color.z);

    let r = (256.0 * r.clamp(0.0, 0.999)) as i32;
    let g = (256.0 * g.clamp(0.0, 0.999)) as i32;
    let b = (256.0 * b.clamp(0.0, 0.999)) as i32;

    println!("{r} {g} {b}");
}
