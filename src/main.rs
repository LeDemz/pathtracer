use indicatif::{ProgressBar, ProgressStyle};
use pathtracer::Color;
use pathtracer::HitRecord;
use pathtracer::Hittable;
use pathtracer::Interval;
use pathtracer::Point3;
use pathtracer::Ray;
use pathtracer::Sphere;
use pathtracer::Vec3;
use pathtracer::INFINITY;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;

fn ray_color<T: Hittable>(r: &Ray, world: &T) -> Color {
    let mut rec: HitRecord = HitRecord {
        p: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 0.0),
        t: 0.0,
        front_face: false,
    };
    if world.hit_interval(r, Interval::new(0.0, INFINITY), &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
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

    // World
    let mut world = pathtracer::HittableList::new();
    let sphere1 = Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    let sphere2 = Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));
    world.add(sphere1);
    world.add(sphere2);

    // Camera
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

            let pixel_color = ray_color(&r, &world);
            pathtracer::write_color(&file, &pixel_color);
        }
    }
    pb.finish_with_message("Done");
}
