use crate::Vec3;
use std::fs::File;
use std::io::Write;

pub type Color = Vec3;

pub fn write_color(mut file_handle: &File, pixel_color: &Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let rbyte: i32 = (255.99 * r).floor() as i32;
    let gbyte: i32 = (255.99 * g).floor() as i32;
    let bbyte: i32 = (255.99 * b).floor() as i32;

    writeln!(file_handle, "{} {} {}", rbyte, gbyte, bbyte)
        .expect("Unable to write content to file");
}
