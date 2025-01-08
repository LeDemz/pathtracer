use indicatif::{ProgressBar, ProgressStyle};
use pathtracer::Color;
use pathtracer::Point3;
use pathtracer::Ray;
use pathtracer::Vec3;
use std::fs::File;
use std::io::Write;

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = *center - (r.origin());
    let a = pathtracer::dot(r.direction(), r.direction());
    let b = -2.0 * pathtracer::dot(r.direction(), oc);
    let c = pathtracer::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    return discriminant >= 0.0;
}

fn ray_color(r: &Ray) -> Color {
    if (hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r)) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = pathtracer::unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    // Calculate the image_height, and ensure that it's a least 1
    let mut image_height: u32 = ((image_width as f64) / aspect_ratio).floor() as u32;
    image_height = if image_height < 1 { 1 } else { image_height };

    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64) / (image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);

    // Calculate the location of the upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut file = File::create("image.ppm").expect("Unable to create file");

    // Indicatif setup
    let total = image_height as u64;

    // Create a progress bar
    let pb = ProgressBar::new(total);

    // Customize the style
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({percent}%)",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    // Render
    writeln!(file, "P3\n{} {}\n255", image_width, image_height)
        .expect("Unable to write header to file");
    for j in 0..image_height {
        pb.inc(1);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r);
            pathtracer::write_color(&file, &pixel_color);
        }
    }
    pb.finish_with_message("Done");
}
