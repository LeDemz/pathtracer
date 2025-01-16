use crate::{Interval, Vec3};
use std::fs::File;
use std::io::Write;

pub type Color = Vec3;

pub fn write_color(mut file_handle: &File, pixel_color: &Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Translate the [0,1] component values to the byte range [0,255]
    let intensity = Interval::new(0.000,0.999);
    let rbyte: i32 = (256.0 * intensity.clamp(r)) as i32;
    let gbyte: i32 = (256.0 * intensity.clamp(g)) as i32;
    let bbyte: i32 = (256.0 * intensity.clamp(b)) as i32;

    writeln!(file_handle, "{} {} {}", rbyte, gbyte, bbyte)
        .expect("Unable to write content to file");
}
